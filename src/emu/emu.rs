use crate::auxiliary::clock::spin_wait;
use crate::auxiliary::joypad::Joypad;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::emu::battery::BatterySave;
use crate::emu::config::EmuConfig;
use crate::emu::ctx::{EmuCtx, EmuState, RunMode};
use crate::emu::save_state::EmuSaveState;
use crate::ui::events::SaveStateEvent;
use crate::ui::Ui;
use crate::CYCLES_PER_FRAME;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{fs, thread};

const _CYCLES_PER_SECOND: usize = 4_194_304;
const CYCLE_TIME: f64 = 238.4185791; // 1 / 4_194_304 seconds ≈ 238.41858 nanoseconds

pub struct Emu {
    pub ctx: EmuCtx,
    pub cpu: Cpu,
    pub ui: Ui,
}

impl Emu {
    pub fn new(config: EmuConfig) -> Result<Self, String> {
        Ok(Self {
            cpu: Cpu::new(Bus::with_bytes(vec![])),
            ui: Ui::new(config.graphics.clone(), false)?,
            ctx: EmuCtx::new(config),
        })
    }

    fn calc_emulated_time(&mut self) -> Duration {
        let speed_multiplier = if let EmuState::Running(mode) = &self.ctx.state {
            match mode {
                RunMode::Rewind | RunMode::Normal => 1.0,
                RunMode::Slow => self.ctx.config.emulation.slow_speed / 100.0,
                RunMode::Turbo => self.ctx.config.emulation.turbo_speed / 100.0,
            }
        } else {
            1.0
        };

        if self.ctx.speed_multiplier != speed_multiplier {
            self.ctx.clock.reset();
        }

        self.ctx.speed_multiplier = speed_multiplier;

        let emulated_time_ns =
            (self.ctx.clock.t_cycles as f64 * CYCLE_TIME / speed_multiplier).round() as u64;

        Duration::from_nanos(emulated_time_ns)
    }

    fn tick(&mut self) -> Result<(), String> {
        let prev_m_cycles = self.ctx.clock.get_m_cycles();

        while self.ctx.clock.get_m_cycles() - prev_m_cycles < CYCLES_PER_FRAME {
            self.cpu.step(&mut self.ctx)?;

            if let Some(debugger) = self.ctx.debugger.as_mut() {
                if !debugger.get_serial_msg().is_empty() {
                    println!("Serial: {}", debugger.get_serial_msg());
                }
            }

            if !self.ctx.config.emulation.is_muted
                && EmuState::Running(RunMode::Normal) == self.ctx.state
            {
                self.ui.audio.update(&mut self.cpu.bus.io.apu)?;
            }
        }

        let real_elapsed = self.ctx.clock.start_time.elapsed();
        let emulated_time = self.calc_emulated_time();

        if emulated_time > real_elapsed {
            spin_wait(emulated_time - real_elapsed);
        }

        if self.ctx.prev_frame != self.ctx.ppu.current_frame {
            self.ui.draw(&mut self.ctx.ppu, &self.cpu.bus);
        }

        self.ctx.prev_frame = self.ctx.ppu.current_frame;

        Ok(())
    }

    pub fn run(&mut self, cart_path: Option<PathBuf>) -> Result<(), String> {
        if let Some(cart_path) = &self.ctx.config.last_cart_path {
            if Path::new(cart_path).exists() {
                self.ctx.state = EmuState::LoadCart(cart_path.into());
            }
        }

        if let Some(cart_path) = cart_path {
            self.ctx.state = EmuState::LoadCart(cart_path);
        }

        loop {
            if self.ctx.state == EmuState::Paused || self.ctx.state == EmuState::WaitCart {
                let text = if self.ctx.state == EmuState::Paused {
                    "PAUSED"
                } else {
                    "DROP FILE"
                };
                self.ui.draw_text(text);
                self.ui.handle_events(&mut self.cpu.bus, &mut self.ctx);
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            if self.ctx.state == EmuState::Quit {
                self.ctx.config.save().map_err(|e| e.to_string())?;
                break;
            }

            if let EmuState::LoadCart(path) = &self.ctx.state {
                let cart = read_cart(path).map_err(|e| e.to_string())?;

                let mut bus = Bus::new(cart);
                bus.io.lcd.set_pallet(self.ui.curr_palette);
                self.cpu = Cpu::new(bus);

                self.ctx.config.last_cart_path = Some(path.to_string_lossy().to_string());
                self.ctx.state = EmuState::Running(RunMode::Normal);
                self.ctx.reset();

                if self.ctx.config.load_save_state_at_start {
                    let name = self.ctx.config.get_last_cart_file_stem().unwrap();
                    let save_state = EmuSaveState::load_file(&name, 0);

                    if let Ok(save_state) = save_state {
                        load_save_state(self, save_state);
                    } else {
                        eprintln!("Failed load save_state: {:?}", save_state);
                    };
                }
            }

            if let EmuState::Running(RunMode::Rewind) = &self.ctx.state {
                if let Some(state) = self.ctx.rewind_buffer.pop_back() {
                    load_save_state(self, state);
                    thread::sleep(Duration::from_millis(100));
                }
            }

            self.ui.handle_events(&mut self.cpu.bus, &mut self.ctx);
            self.tick()?;

            let now = Instant::now();
            if self.ctx.config.emulation.rewind_size > 0
                && now.duration_since(self.ctx.last_rewind_save).as_secs_f32() >= 2.0
            {
                if self.ctx.rewind_buffer.len() > self.ctx.config.emulation.rewind_size {
                    self.ctx.rewind_buffer.pop_front();
                }

                self.ctx
                    .rewind_buffer
                    .push_back(self.create_save_state(&self.cpu));
                self.ctx.last_rewind_save = now;
            }

            if let Some((event, index)) = self.ctx.pending_save_state.take() {
                let name = self.ctx.config.get_last_cart_file_stem().unwrap();

                match event {
                    SaveStateEvent::Create => {
                        let save_state = self.create_save_state(&self.cpu);

                        if let Err(err) = save_state.save_file(&name, index) {
                            eprintln!("Failed save_state: {:?}", err);
                        }
                    }
                    SaveStateEvent::Load => {
                        let save_state = EmuSaveState::load_file(&name, index);

                        let Ok(save_state) = save_state else {
                            eprintln!("Failed load save_state: {:?}", save_state);
                            continue;
                        };

                        load_save_state(self, save_state);
                    }
                }
            }
        }

        let name = self.ctx.config.get_last_cart_file_stem().unwrap();

        if let Some(bytes) = self.cpu.bus.cart.dump_ram() {
            BatterySave::from_bytes(bytes)
                .save_file(&name)
                .map_err(|e| e.to_string())?;
        }

        if let Err(err) = self.create_save_state(&self.cpu).save_file(&name, 0) {
            eprintln!("Failed save_state: {:?}", err);
        }

        Ok(())
    }

    pub fn create_save_state(&self, cpu: &Cpu) -> EmuSaveState {
        EmuSaveState {
            cpu_without_bus: cpu.clone_without_bus(),
            bus_without_cart: cpu.bus.clone_without_cart(),
            cart_mbc: cpu.bus.cart.mbc.clone(),
        }
    }
}

fn load_save_state(emu: &mut Emu, save_state: EmuSaveState) {
    let mut state_cpu = save_state.cpu_without_bus; // reconstruct cpu
    state_cpu.bus = save_state.bus_without_cart;
    state_cpu.bus.io.joypad = Joypad::default(); // reset controls
    state_cpu.bus.cart.mbc = save_state.cart_mbc; // reconstruct cart
    state_cpu.bus.cart.data = emu.cpu.bus.cart.data.clone();

    emu.cpu = state_cpu;
    emu.ctx.reset();
}

pub fn read_cart(file_path: &Path) -> Result<Cart, String> {
    let bytes = read_bytes(file_path).map_err(|e| e.to_string())?;
    let mut cart = Cart::new(bytes).map_err(|e| e.to_string())?;
    _ = print_cart(&cart).map_err(|e| println!("Failed to print cart: {}", e));
    let file_name = file_path
        .file_stem()
        .expect("we read file")
        .to_str()
        .unwrap();

    let Ok(save) = BatterySave::load_file(file_name) else {
        return Ok(cart);
    };

    cart.load_ram(save.ram_bytes);

    Ok(cart)
}

fn print_cart(cart: &Cart) -> Result<(), String> {
    println!("Cart Loaded:");
    println!("\t Title          : {}", cart.data.get_title());
    println!("\t Type           : {:?}", cart.data.get_cart_type()?);
    println!("\t ROM Size       : {:?}", cart.data.get_rom_size()?);
    println!("\t ROM bytes       : {:?}", cart.data.bytes.len());
    println!("\t RAM Size       : {:?}", cart.data.get_ram_size()?);
    println!("\t ROM Version    : {:02X}", cart.data.get_rom_version());
    println!("\t Checksum Valid : {}", cart.data.checksum_valid());

    Ok(())
}

pub fn read_bytes(file_path: &Path) -> Result<Box<[u8]>, String> {
    if !file_path.exists() {
        return Err(format!("File not found: {:?}", file_path));
    }

    fs::read(file_path)
        .map(|x| x.into_boxed_slice())
        .map_err(|e| format!("Failed to read file: {}", e))
}
