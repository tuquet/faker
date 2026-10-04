// App State
let currentUsers = [];
let filteredUsers = [];
let currentTab = 'cards';

// DOM Elements
const inputCount = document.getElementById('inputCount');
const selectGender = document.getElementById('selectGender');
const selectNat = document.getElementById('selectNat');
const selectMode = document.getElementById('selectMode');
const btnGenerate = document.getElementById('btnGenerate');
const btnExportCSV = document.getElementById('btnExportCSV');
const btnExportJSON = document.getElementById('btnExportJSON');
const btnCopyJSON = document.getElementById('btnCopyJSON');
const filterInput = document.getElementById('filterInput');

const tabCards = document.getElementById('tabCards');
const tabTable = document.getElementById('tabTable');
const tabJSON = document.getElementById('tabJSON');

const cardsContainer = document.getElementById('cardsContainer');
const tableContainer = document.getElementById('tableContainer');
const tableBody = document.getElementById('tableBody');
const jsonContainer = document.getElementById('jsonContainer');
const rawJsonBlock = document.getElementById('rawJsonBlock');
const loadingState = document.getElementById('loadingState');

const resultCount = document.getElementById('resultCount');
const loadTime = document.getElementById('loadTime');
const sourceInfo = document.getElementById('sourceInfo');
const toast = document.getElementById('toast');
const toastMsg = document.getElementById('toastMsg');

// Toast Notification
function showToast(message) {
  toastMsg.textContent = message;
  toast.classList.remove('opacity-0', 'translate-y-10', 'pointer-events-none');
  setTimeout(() => {
    toast.classList.add('opacity-0', 'translate-y-10', 'pointer-events-none');
  }, 2200);
}

// Copy to Clipboard
function copyToClipboard(text, label = 'nội dung') {
  if (!text) return;
  navigator.clipboard.writeText(text).then(() => {
    showToast(`Đã sao chép ${label}!`);
  }).catch(() => {
    const el = document.createElement('textarea');
    el.value = text;
    document.body.appendChild(el);
    el.select();
    document.execCommand('copy');
    document.body.removeChild(el);
    showToast(`Đã sao chép ${label}!`);
  });
}

// Switch Tabs
function setTab(tab) {
  currentTab = tab;
  tabCards.classList.toggle('active', tab === 'cards');
  tabTable.classList.toggle('active', tab === 'table');
  tabJSON.classList.toggle('active', tab === 'json');

  cardsContainer.classList.toggle('hidden', tab !== 'cards');
  tableContainer.classList.toggle('hidden', tab !== 'table');
  jsonContainer.classList.toggle('hidden', tab !== 'json');
}

// Fetch Random Users
async function fetchUsers() {
  const count = parseInt(inputCount.value, 10) || 10;
  const gender = selectGender.value;
  const nat = selectNat.value === 'all' ? '' : selectNat.value;
  const mode = selectMode.value;

  loadingState.classList.remove('hidden');
  cardsContainer.classList.add('hidden');
  tableContainer.classList.add('hidden');
  jsonContainer.classList.add('hidden');

  const startTime = performance.now();

  try {
    const url = new URL('/api/users', window.location.origin);
    url.searchParams.set('results', count);
    if (gender) url.searchParams.set('gender', gender);
    if (nat) url.searchParams.set('nat', nat);
    if (mode) url.searchParams.set('mode', mode);

    const res = await fetch(url);
    if (!res.ok) throw new Error(`HTTP error! status: ${res.status}`);

    const data = await res.json();
    const endTime = performance.now();

    currentUsers = data.results || [];
    filteredUsers = [...currentUsers];

    // Update Stats
    const elapsed = Math.round(endTime - startTime);
    loadTime.textContent = `${elapsed} ms`;
    resultCount.textContent = `${currentUsers.length} profiles`;
    sourceInfo.textContent = data.source || (data.offlineFallback ? 'Offline' : 'API');
    if (data.offlineFallback) {
      sourceInfo.className = 'px-2 py-0.5 rounded bg-amber-100 text-amber-800 font-mono text-[11px]';
    } else {
      sourceInfo.className = 'px-2 py-0.5 rounded bg-blue-100 text-blue-800 font-mono text-[11px]';
    }

    renderData();
  } catch (err) {
    console.error('Fetch error:', err);
    showToast('Lỗi khi tải dữ liệu! Vui lòng thử lại.');
  } finally {
    loadingState.classList.add('hidden');
    setTab(currentTab);
  }
}

// Render Data
function renderData() {
  renderCards(filteredUsers);
  renderTable(filteredUsers);
  renderJSON(filteredUsers);
}

// Render Cards View
function renderCards(users) {
  if (!users.length) {
    cardsContainer.innerHTML = `
      <div class="col-span-full py-12 text-center text-slate-400">
        <i class="fa-regular fa-folder-open text-4xl mb-2"></i>
        <p>Không tìm thấy người dùng nào phù hợp.</p>
      </div>`;
    return;
  }

  cardsContainer.innerHTML = users.map((u, idx) => {
    const name = `${u.name?.title || ''} ${u.name?.first || ''} ${u.name?.last || ''}`.trim();
    const street = u.location?.street ? `${u.location.street.number || ''} ${u.location.street.name || ''}`.trim() : '';
    const cityState = `${u.location?.city || ''}, ${u.location?.state || ''}`.trim();
    const country = u.location?.country || u.nat || '';
    const email = u.email || '';
    const phone = u.phone || '';
    const username = u.login?.username || '';
    const password = u.login?.password || '';
    const avatar = u.picture?.large || 'https://via.placeholder.com/150';
    const age = u.dob?.age || '';
    const genderIcon = u.gender === 'male' ? '<i class="fa-solid fa-mars text-blue-500"></i>' : '<i class="fa-solid fa-venus text-pink-500"></i>';

    return `
      <div class="profile-card bg-white rounded-2xl p-5 border border-slate-200 shadow-sm flex flex-col justify-between">
        <div>
          <!-- Header: Avatar + Main Name -->
          <div class="flex items-start space-x-3.5 pb-4 border-b border-slate-100">
            <img src="${avatar}" alt="${name}" class="w-14 h-14 rounded-2xl object-cover ring-2 ring-slate-100 shadow-sm">
            <div class="flex-1 min-w-0">
              <div class="flex items-center justify-between">
                <span class="inline-flex items-center gap-1 text-xs font-semibold px-2 py-0.5 rounded-md bg-slate-100 text-slate-600">
                  ${genderIcon} ${age ? age + ' tuổi' : ''}
                </span>
                <span class="text-xs font-bold text-slate-400 font-mono">#${idx + 1}</span>
              </div>
              <h3 class="font-bold text-slate-900 text-sm truncate mt-1" title="${name}">${name}</h3>
              <p class="text-xs text-slate-500 truncate"><i class="fa-solid fa-location-dot text-slate-400 mr-1"></i>${cityState}, ${country}</p>
            </div>
          </div>

          <!-- Body Fields -->
          <div class="py-3 space-y-2 text-xs">
            <!-- Email -->
            <div class="copyable-field p-2 rounded-lg flex items-center justify-between group" onclick="copyToClipboard('${email}', 'Email')">
              <div class="flex items-center space-x-2 truncate">
                <i class="fa-regular fa-envelope text-slate-400 w-4"></i>
                <span class="text-slate-700 truncate">${email}</span>
              </div>
              <i class="fa-regular fa-copy text-slate-300 group-hover:text-blue-600 ml-2"></i>
            </div>

            <!-- Phone -->
            <div class="copyable-field p-2 rounded-lg flex items-center justify-between group" onclick="copyToClipboard('${phone}', 'Số điện thoại')">
              <div class="flex items-center space-x-2 truncate">
                <i class="fa-solid fa-phone text-slate-400 w-4"></i>
                <span class="text-slate-700 truncate">${phone}</span>
              </div>
              <i class="fa-regular fa-copy text-slate-300 group-hover:text-blue-600 ml-2"></i>
            </div>

            <!-- Address -->
            <div class="copyable-field p-2 rounded-lg flex items-center justify-between group" onclick="copyToClipboard('${street}, ${cityState}, ${country}', 'Địa chỉ')">
              <div class="flex items-center space-x-2 truncate">
                <i class="fa-solid fa-house text-slate-400 w-4"></i>
                <span class="text-slate-700 truncate">${street || 'N/A'}</span>
              </div>
              <i class="fa-regular fa-copy text-slate-300 group-hover:text-blue-600 ml-2"></i>
            </div>

            <!-- Username / Password Box -->
            <div class="bg-slate-50 p-2.5 rounded-xl border border-slate-100 flex items-center justify-between mt-2 font-mono text-[11px]">
              <div class="truncate mr-2">
                <span class="text-slate-400">User:</span> <strong class="text-slate-800">${username}</strong>
                <br>
                <span class="text-slate-400">Pass:</span> <span class="text-emerald-700 font-bold">${password}</span>
              </div>
              <button onclick="copyToClipboard('${password}', 'Mật khẩu')" title="Copy Password" class="px-2 py-1 rounded bg-white border border-slate-200 hover:bg-slate-100 text-slate-600 shadow-2xs">
                <i class="fa-solid fa-key"></i>
              </button>
            </div>
          </div>
        </div>

        <!-- Card Footer -->
        <div class="pt-2 border-t border-slate-100 flex items-center justify-between text-[11px] text-slate-400">
          <span>Nat: <strong class="text-slate-600">${u.nat || 'Global'}</strong></span>
          <button onclick="copyToClipboard(JSON.stringify(currentUsers[${idx}], null, 2), 'JSON của profile')" class="hover:text-blue-600 transition">
            <i class="fa-solid fa-code"></i> Copy Object
          </button>
        </div>
      </div>
    `;
  }).join('');
}

// Render Table View
function renderTable(users) {
  if (!users.length) {
    tableBody.innerHTML = `<tr><td colspan="9" class="p-6 text-center text-slate-400">Không có dữ liệu</td></tr>`;
    return;
  }

  tableBody.innerHTML = users.map((u, idx) => {
    const name = `${u.name?.first || ''} ${u.name?.last || ''}`;
    const street = u.location?.street ? `${u.location.street.number || ''} ${u.location.street.name || ''}` : '';
    const loc = `${street}, ${u.location?.city || ''} (${u.location?.country || u.nat})`;
    const avatar = u.picture?.thumbnail || u.picture?.large;

    return `
      <tr class="hover:bg-slate-50 transition">
        <td class="px-4 py-2.5">
          <img src="${avatar}" alt="" class="w-8 h-8 rounded-full object-cover border border-slate-200">
        </td>
        <td class="px-4 py-2.5 font-semibold text-slate-900">${name}</td>
        <td class="px-4 py-2.5 capitalize">${u.gender}</td>
        <td class="px-4 py-2.5 font-mono text-slate-600">
          <span class="cursor-pointer hover:underline" onclick="copyToClipboard('${u.email}', 'Email')">${u.email}</span>
        </td>
        <td class="px-4 py-2.5 font-mono">${u.phone}</td>
        <td class="px-4 py-2.5 truncate max-w-xs" title="${loc}">${loc}</td>
        <td class="px-4 py-2.5 font-mono text-slate-700">${u.login?.username || ''}</td>
        <td class="px-4 py-2.5 font-mono text-emerald-700 font-bold">
          <span class="cursor-pointer hover:underline" onclick="copyToClipboard('${u.login?.password}', 'Mật khẩu')">${u.login?.password || ''}</span>
        </td>
        <td class="px-4 py-2.5 text-right">
          <button class="px-2 py-1 rounded bg-slate-100 hover:bg-slate-200 text-slate-700" onclick="copyToClipboard(JSON.stringify(currentUsers[${idx}], null, 2), 'JSON')">
            <i class="fa-regular fa-copy"></i>
          </button>
        </td>
      </tr>
    `;
  }).join('');
}

// Render JSON
function renderJSON(users) {
  rawJsonBlock.textContent = JSON.stringify(users, null, 2);
}

// Export to CSV
function exportCSV() {
  if (!currentUsers.length) {
    showToast('Chưa có dữ liệu để xuất!');
    return;
  }

  fetch('/api/export/csv', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ users: currentUsers })
  })
  .then(res => res.blob())
  .then(blob => {
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `random-users-${Date.now()}.csv`;
    document.body.appendChild(a);
    a.click();
    a.remove();
    showToast('Đã tải xuống file CSV!');
  })
  .catch(err => {
    console.error('Export CSV error:', err);
    showToast('Lỗi khi xuất file CSV!');
  });
}

// Export to JSON
function exportJSON() {
  if (!currentUsers.length) {
    showToast('Chưa có dữ liệu để xuất!');
    return;
  }
  const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(currentUsers, null, 2));
  const a = document.createElement('a');
  a.href = dataStr;
  a.download = `random-users-${Date.now()}.json`;
  document.body.appendChild(a);
  a.click();
  a.remove();
  showToast('Đã tải xuống file JSON!');
}

// Client-side quick filter
filterInput.addEventListener('input', (e) => {
  const query = e.target.value.toLowerCase().trim();
  if (!query) {
    filteredUsers = [...currentUsers];
  } else {
    filteredUsers = currentUsers.filter(u => {
      const full = `${u.name?.first || ''} ${u.name?.last || ''} ${u.email || ''} ${u.phone || ''} ${u.location?.country || ''} ${u.login?.username || ''}`.toLowerCase();
      return full.includes(query);
    });
  }
  renderData();
});

// Event Listeners
btnGenerate.addEventListener('click', fetchUsers);
btnExportCSV.addEventListener('click', exportCSV);
btnExportJSON.addEventListener('click', exportJSON);
btnCopyJSON.addEventListener('click', () => {
  copyToClipboard(JSON.stringify(currentUsers, null, 2), 'toàn bộ JSON');
});

tabCards.addEventListener('click', () => setTab('cards'));
tabTable.addEventListener('click', () => setTab('table'));
tabJSON.addEventListener('click', () => setTab('json'));

// Quick count buttons
document.querySelectorAll('.quick-count').forEach(btn => {
  btn.addEventListener('click', () => {
    inputCount.value = btn.dataset.val;
    document.querySelectorAll('.quick-count').forEach(b => {
      b.classList.remove('bg-blue-100', 'text-blue-700', 'font-bold');
      b.classList.add('bg-slate-100', 'text-slate-700');
    });
    btn.classList.add('bg-blue-100', 'text-blue-700', 'font-bold');
    btn.classList.remove('bg-slate-100', 'text-slate-700');
    fetchUsers();
  });
});

// Initial Load
window.addEventListener('DOMContentLoaded', () => {
  fetchUsers();
});
