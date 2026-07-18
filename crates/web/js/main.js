import init, { oxGBC } from '../pkg/oxgbc_web.js';
import { AudioScheduler } from './audio.js';

const KEYMAP = {
  ArrowUp: 'up', ArrowDown: 'down', ArrowLeft: 'left', ArrowRight: 'right',
  KeyX: 'a', KeyZ: 'b',
  Enter: 'start', Backspace: 'select',
};

// Bundled open-source demo ROMs shown in the ROM dropdown.
const DEMOS = {
  tobudx: { url: './assets/tobudx.gb', name: 'Tobu Tobu Girl DX' },
  ucity: { url: './assets/ucity.gbc', name: 'µCity' },
  geometrix: { url: './assets/geometrix.gbc', name: 'Geometrix' },
  libbet: { url: './assets/libbet.gb', name: 'Libbet & the Magic Floor' },
  dmgacid2: { url: './assets/dmg-acid2.gb', name: 'dmg-acid2' },
  cgbacid2: { url: './assets/cgb-acid2.gbc', name: 'cgb-acid2' },
  cgbacidhell: { url: './assets/cgb-acid-hell.gbc', name: 'cgb-acid-hell' },
  cpuinstrs: { url: './assets/cpu_instrs.gb', name: 'cpu_instrs' },
};

// Real Game Boy frame rate (4194304 Hz / 70224 cycles ≈ 59.7275 Hz). Stepped with
// an accumulator so speed is independent of the display refresh rate.
const GB_FPS = 4194304 / 70224;
const FRAME_MS = 1000 / GB_FPS;

async function main() {
  await init();
  const gb = new oxGBC();
  const audio = new AudioScheduler(gb.sample_rate());

  const $ = (id) => document.getElementById(id);
  const canvas = $('screen');
  const ctx = canvas.getContext('2d');
  const w = gb.width(), h = gb.height();
  canvas.width = w; canvas.height = h;
  const img = new ImageData(w, h);

  const loader = $('loader');
  const pauseOverlay = $('pause-overlay');
  const fileName = $('fileName');

  let romLoaded = false;
  let paused = false;
  let audioOn = false;
  let scale = 3;

  function applyScale() {
    canvas.style.width = (w * scale) + 'px';
    $('scaleInput').value = scale;
  }
  applyScale();

  function loadRom(bytes, label) {
    try {
      gb.load_rom(bytes);
      romLoaded = true;
      paused = false;
      pauseOverlay.classList.remove('active');
      fileName.textContent = '';   // name hidden; span reused only for load errors
      loader.classList.add('hidden');
    } catch (err) {
      romLoaded = false;
      fileName.textContent = '⚠ ' + err;
    }
  }

  async function loadDemo(key) {
    const demo = DEMOS[key];
    if (!demo) return;
    loader.classList.remove('hidden');
    try {
      const res = await fetch(demo.url);
      loadRom(new Uint8Array(await res.arrayBuffer()), demo.name);
    } catch (_) {
      fileName.textContent = '⚠ load failed';
      loader.classList.add('hidden');
    }
  }

  // --- Controls: ROM select / FILE ---
  $('roms').addEventListener('change', (e) => loadDemo(e.target.value));
  $('romInput').addEventListener('change', async (e) => {
    const file = e.target.files[0];
    if (!file) return;
    loadRom(new Uint8Array(await file.arrayBuffer()), file.name);
  });

  // --- Pause ---
  const pauseBtn = $('pause');
  pauseBtn.addEventListener('click', () => {
    if (!romLoaded) return;
    paused = !paused;
    pauseOverlay.classList.toggle('active', paused);
    $('icon-pause').style.display = paused ? 'none' : 'inline';
    $('icon-play').style.display = paused ? 'inline' : 'none';
  });

  // --- Audio toggle (also the user gesture that unlocks AudioContext) ---
  const audioBtn = $('audio');
  function setAudio(on) {
    audioOn = on;
    if (on) audio.resume();
    audioBtn.classList.toggle('off', !on);
    $('icon-audio-on').style.display = on ? 'inline' : 'none';
    $('icon-audio-off').style.display = on ? 'none' : 'inline';
  }
  audioBtn.addEventListener('click', () => setAudio(!audioOn));

  // --- Scale ---
  $('scaleMinus').addEventListener('click', () => { scale = Math.max(1, scale - 1); applyScale(); });
  $('scalePlus').addEventListener('click', () => { scale = Math.min(6, scale + 1); applyScale(); });

  // --- Shell color theme (persisted) ---
  const device = document.querySelector('.device');
  const swatches = document.querySelectorAll('.swatch');
  function setTheme(t) {
    device.dataset.theme = t;
    try { localStorage.setItem('oxgbc-theme', t); } catch (_) {}
    for (const s of swatches) s.classList.toggle('active', s.dataset.theme === t);
  }
  for (const s of swatches) s.addEventListener('click', () => setTheme(s.dataset.theme));
  let savedTheme = 'atomic';
  try { savedTheme = localStorage.getItem('oxgbc-theme') || 'atomic'; } catch (_) {}
  setTheme(savedTheme);

  // --- Keyboard ---
  addEventListener('keydown', (e) => {
    const b = KEYMAP[e.code];
    if (b) { gb.set_button(b, true); e.preventDefault(); }
  });
  addEventListener('keyup', (e) => {
    const b = KEYMAP[e.code];
    if (b) { gb.set_button(b, false); e.preventDefault(); }
  });

  // --- On-screen buttons (mouse + touch) ---
  for (const el of document.querySelectorAll('[data-btn]')) {
    const name = el.dataset.btn;
    const press = (e) => { e.preventDefault(); gb.set_button(name, true); el.classList.add('pressed'); el.setPointerCapture?.(e.pointerId); };
    const release = (e) => { e.preventDefault(); gb.set_button(name, false); el.classList.remove('pressed'); };
    el.addEventListener('pointerdown', press);
    el.addEventListener('pointerup', release);
    el.addEventListener('pointercancel', release);
    el.addEventListener('pointerleave', release);
    el.addEventListener('contextmenu', (e) => e.preventDefault());
  }

  // --- Boot the default demo ---
  await loadDemo('tobudx');

  // --- Run loop ---
  let acc = 0;
  let last = null;
  function loop(now) {
    if (last === null) last = now;
    acc += now - last;
    last = now;

    if (romLoaded && !paused) {
      if (acc > 100) acc = 100;
      let ran = false;
      while (acc >= FRAME_MS) {
        gb.run_frame();
        const samples = gb.take_audio(); // always drain the APU buffer
        if (audioOn) audio.push(samples);
        acc -= FRAME_MS;
        ran = true;
      }
      if (ran) { img.data.set(gb.frame_buffer()); ctx.putImageData(img, 0, 0); }
    } else {
      acc = 0;
    }
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);
}

main();
