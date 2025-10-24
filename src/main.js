const { open } = window.__TAURI__.dialog;
const { getCurrentWebview } = window.__TAURI__.webview;

const dropArea = document.getElementById('dropArea');
const convertBtn = document.getElementById('convertBtn');
const statusEl = document.getElementById('status');

let selectedPaths = [];

dropArea.addEventListener('click', async () => {
  const paths = await open({
    multiple: true,
    filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp'] }]
  });
  if (paths) {
    selectedPaths = Array.isArray(paths) ? paths : [paths];
    updateUI();
  }
});

const unlisten = getCurrentWebview().onDragDropEvent((event) => {
  if (event.payload.type === 'over') {
    dropArea.classList.add('drag-over');
  } else if (event.payload.type === 'leave') {
    dropArea.classList.remove('drag-over');
  } else if (event.payload.type === 'drop') {
    dropArea.classList.remove('drag-over');
    selectedPaths = event.payload.paths;
    updateUI();
  }
});

window.addEventListener('beforeunload', unlisten);

function updateUI() {
  const count = selectedPaths.length;
  if (count > 0) {
    statusEl.textContent = `Выбрано файлов: ${count}`;
    convertBtn.disabled = false;
  } else {
    statusEl.textContent = 'Статус: Готово к конвертации';
    convertBtn.disabled = true;
  }
}

convertBtn.addEventListener('click', async () => {
  if (selectedPaths.length === 0) return;

  statusEl.textContent = 'Конвертация...';
  convertBtn.disabled = true;

  try {
    const result = await window.__TAURI__.core.invoke('convert_images_to_jpeg', {
      paths: selectedPaths
    });
    statusEl.textContent = `Готово! Сохранено файлов: ${result.length}`;
    console.log('Результат:', result);
  } catch (err) {
    console.error('Ошибка:', err);
    statusEl.textContent = 'Ошибка: ' + (err.message || err);
  } finally {
    convertBtn.disabled = false;
  }
});