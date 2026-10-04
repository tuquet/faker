// Encapsulate in IIFE to prevent global identifier collisions on refresh or injection
(function () {
  'use strict';

  // Universal Tauri Invoke Resolver
  function getTauriInvoke() {
    if (typeof window !== 'undefined') {
      if (window.__TAURI_INTERNALS__ && typeof window.__TAURI_INTERNALS__.invoke === 'function') {
        return (cmd, payload) => window.__TAURI_INTERNALS__.invoke(cmd, payload);
      }
      if (window.__TAURI__ && window.__TAURI__.core && typeof window.__TAURI__.core.invoke === 'function') {
        return (cmd, payload) => window.__TAURI__.core.invoke(cmd, payload);
      }
      if (window.__TAURI__ && typeof window.__TAURI__.invoke === 'function') {
        return (cmd, payload) => window.__TAURI__.invoke(cmd, payload);
      }
    }
    return null;
  }

  const isTauriEnv = getTauriInvoke() !== null;

  // App State
  let currentUsers = [];
  let filteredUsers = [];
  let currentTab = 'cards';
  let logCount = 0;
  let logHistory = [];

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
  const statusBadge = document.getElementById('statusBadge');
  const toast = document.getElementById('toast');
  const toastMsg = document.getElementById('toastMsg');

  // Log Elements
  const logContent = document.getElementById('logContent');
  const logCounter = document.getElementById('logCounter');
  const btnClearLog = document.getElementById('btnClearLog');
  const btnCopyLog = document.getElementById('btnCopyLog');
  const btnToggleLog = document.getElementById('btnToggleLog');

  // Unified Logger (Screen + File app.log)
  function addLog(level, message) {
    const now = new Date();
    const timeStr = now.toTimeString().split(' ')[0] + '.' + String(now.getMilliseconds()).padStart(3, '0');
    const logLine = `[${timeStr}] [${level}] ${message}`;
    
    logHistory.push(logLine);
    logCount++;
    if (logCounter) logCounter.textContent = `${logCount} sự kiện`;

    if (logContent) {
      const el = document.createElement('div');
      if (level === 'ERROR') {
        el.className = 'text-rose-400 font-bold';
      } else if (level === 'SUCCESS') {
        el.className = 'text-emerald-400';
      } else if (level === 'WARN') {
        el.className = 'text-amber-400';
      } else {
        el.className = 'text-slate-300';
      }
      el.textContent = logLine;
      logContent.appendChild(el);
      logContent.scrollTop = logContent.scrollHeight;
    }

    // Also forward to Rust file logger if in Tauri
    const invoke = getTauriInvoke();
    if (invoke) {
      try {
        invoke('log_client_message', { level, message }).catch(() => {});
      } catch (e) {}
    }
  }

  // Global Exception Catchers
  window.onerror = function (msg, url, line, col, err) {
    addLog('ERROR', `Lỗi JavaScript: ${msg} (${line}:${col})`);
    return false;
  };

  window.onunhandledrejection = function (event) {
    addLog('ERROR', `Promise Rejection: ${event.reason}`);
  };

  // Update Status Badge if running inside Native Tauri
  if (isTauriEnv) {
    if (statusBadge) {
      statusBadge.innerHTML = `
        <span class="w-2 h-2 rounded-full bg-blue-500 animate-pulse"></span>
        Tauri Native v2
      `;
      statusBadge.className = 'inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold bg-blue-50 text-blue-700 border border-blue-200';
    }
    addLog('INFO', 'Phát hiện môi trường: Tauri Native v2 Runtime (kết nối trực tiếp Rust Core)');
  } else {
    addLog('INFO', 'Chạy ở chế độ Browser / Localhost');
  }

  // Toast Notification
  function showToast(message) {
    if (!toast || !toastMsg) return;
    toastMsg.textContent = message;
    toast.classList.remove('opacity-0', 'translate-y-10', 'pointer-events-none');
    setTimeout(() => {
      toast.classList.add('opacity-0', 'translate-y-10', 'pointer-events-none');
    }, 2800);
  }

  // Copy to Clipboard (Exposed to window for inline onclick)
  function copyToClipboard(text, label = 'nội dung') {
    if (!text) return;
    navigator.clipboard.writeText(text).then(() => {
      showToast(`Đã sao chép ${label}!`);
      addLog('INFO', `Đã copy ${label} vào clipboard`);
    }).catch(() => {
      const el = document.createElement('textarea');
      el.value = text;
      document.body.appendChild(el);
      el.select();
      document.execCommand('copy');
      document.body.removeChild(el);
      showToast(`Đã sao chép ${label}!`);
      addLog('INFO', `Đã copy ${label} (fallback) vào clipboard`);
    });
  }
  window.copyToClipboard = copyToClipboard;

  // Switch Tabs
  function setTab(tab) {
    currentTab = tab;
    if (tabCards) tabCards.classList.toggle('active', tab === 'cards');
    if (tabTable) tabTable.classList.toggle('active', tab === 'table');
    if (tabJSON) tabJSON.classList.toggle('active', tab === 'json');

    if (cardsContainer) cardsContainer.classList.toggle('hidden', tab !== 'cards');
    if (tableContainer) tableContainer.classList.toggle('hidden', tab !== 'table');
    if (jsonContainer) jsonContainer.classList.toggle('hidden', tab !== 'json');
  }

  // Built-in JavaScript Offline Generator (Guaranteed safety net)
  function generateClientMockUsers(count, genderFilter, natCode) {
    addLog('INFO', `Tạo ${count} profiles bằng Offline JS Engine (nat: ${natCode || 'US'})`);
    const isVn = (natCode || '').toLowerCase().includes('vn');
    const maleFirst = isVn
      ? ['Minh', 'Hoàng', 'Duy', 'Tuấn', 'Nam', 'Quân', 'Long', 'Đức', 'Anh', 'Hùng', 'Bảo', 'Huy', 'Thành', 'Phúc', 'Việt']
      : ['James', 'John', 'Robert', 'Michael', 'William', 'David', 'Richard', 'Joseph', 'Thomas', 'Charles', 'Daniel', 'Matthew'];
    
    const femaleFirst = isVn
      ? ['Linh', 'Trang', 'Hương', 'Mai', 'Lan', 'Ngọc', 'Hà', 'Phương', 'Thu', 'Thảo', 'Huyền', 'Yến', 'My', 'Hằng', 'Tú']
      : ['Mary', 'Patricia', 'Jennifer', 'Linda', 'Elizabeth', 'Barbara', 'Susan', 'Jessica', 'Sarah', 'Karen', 'Lisa', 'Nancy'];
    
    const lastNames = isVn
      ? ['Nguyễn', 'Trần', 'Lê', 'Phạm', 'Hoàng', 'Huỳnh', 'Phan', 'Vũ', 'Võ', 'Đặng', 'Bùi', 'Đỗ', 'Hồ', 'Ngô', 'Dương']
      : ['Smith', 'Johnson', 'Williams', 'Brown', 'Jones', 'Garcia', 'Miller', 'Davis', 'Rodriguez', 'Martinez', 'Hernandez'];
    
    const cities = isVn
      ? ['Hà Nội', 'TP Hồ Chí Minh', 'Đà Nẵng', 'Hải Phòng', 'Cần Thơ', 'Nha Trang', 'Huế', 'Vũng Tàu', 'Bình Dương']
      : ['New York', 'Los Angeles', 'Chicago', 'Houston', 'Phoenix', 'Philadelphia', 'San Antonio', 'San Diego'];

    const streets = isVn
      ? ['Đường Lê Lợi', 'Đường Nguyễn Huệ', 'Đường Trần Hưng Đạo', 'Đường Hai Bà Trưng', 'Đường Lý Thường Kiệt']
      : ['Main Street', 'Oak Avenue', 'Maple Lane', 'Cedar Drive', 'Pine Court', 'Washington Boulevard'];

    const results = [];
    for (let i = 0; i < count; i++) {
      const isMale = genderFilter === 'male' ? true : (genderFilter === 'female' ? false : Math.random() > 0.5);
      const gender = isMale ? 'male' : 'female';
      const firstList = isMale ? maleFirst : femaleFirst;
      const first = firstList[Math.floor(Math.random() * firstList.length)];
      const last = lastNames[Math.floor(Math.random() * lastNames.length)];
      const title = isMale ? 'Mr' : (Math.random() > 0.5 ? 'Ms' : 'Mrs');
      const street = `${Math.floor(Math.random() * 8999) + 100} ${streets[Math.floor(Math.random() * streets.length)]}`;
      const city = cities[Math.floor(Math.random() * cities.length)];
      const country = isVn ? 'Vietnam' : 'United States';
      const num = Math.floor(Math.random() * 8999) + 1000;
      const username = `${first.toLowerCase()}_${last.toLowerCase()}${num}`.normalize('NFD').replace(/[\u0300-\u036f]/g, '').replace(/đ/g, 'd');
      const email = `${username}@example.com`;
      const password = `Pass_${Math.floor(Math.random() * 8999) + 1000}!`;
      const phone = isVn ? `09${Math.floor(Math.random() * 89999999) + 10000000}` : `(555) 019-${num}`;
      const avatarGender = isMale ? 'men' : 'women';
      const avatarId = Math.floor(Math.random() * 95) + 1;
      const age = Math.floor(Math.random() * 45) + 20;

      results.push({
        gender,
        name: { title, first, last },
        location: {
          street: { number: num, name: street },
          city,
          state: isVn ? city : 'California',
          country,
          postcode: `${Math.floor(Math.random() * 89999) + 10000}`
        },
        email,
        login: {
          uuid: 'uuid-' + Math.random().toString(36).substring(2, 12),
          username,
          password
        },
        dob: { date: `${2026 - age}-05-15T00:00:00.000Z`, age },
        phone,
        cell: phone,
        picture: {
          large: `https://randomuser.me/api/portraits/${avatarGender}/${avatarId}.jpg`,
          medium: `https://randomuser.me/api/portraits/med/${avatarGender}/${avatarId}.jpg`,
          thumbnail: `https://randomuser.me/api/portraits/thumb/${avatarGender}/${avatarId}.jpg`
        },
        nat: isVn ? 'VN' : (natCode || 'US').toUpperCase()
      });
    }

    return {
      results,
      source: 'client-offline-engine',
      offlineFallback: true
    };
  }

  // Fetch Random Users (3-Tier Engine: Tauri Rust -> Direct HTTPS -> Built-in Client Mock)
  async function fetchUsers() {
    const count = parseInt(inputCount?.value, 10) || 10;
    const gender = selectGender?.value || null;
    const nat = selectNat?.value === 'all' ? null : (selectNat?.value || null);
    const mode = selectMode?.value || 'auto';

    addLog('INFO', `▶ Bắt đầu tạo dữ liệu: count=${count}, gender=${gender || 'all'}, nat=${nat || 'all'}, mode=${mode}`);

    if (loadingState) loadingState.classList.remove('hidden');
    if (cardsContainer) cardsContainer.classList.add('hidden');
    if (tableContainer) tableContainer.classList.add('hidden');
    if (jsonContainer) jsonContainer.classList.add('hidden');

    const startTime = performance.now();
    let data = null;

    // Tier 1: Try Native Tauri IPC
    const invoke = getTauriInvoke();
    if (invoke) {
      try {
        addLog('INFO', 'Đang gọi lệnh fetch_users xuống Rust Core qua Tauri IPC...');
        data = await invoke('fetch_users', {
          count: count,
          gender: gender,
          nat: nat,
          mode: mode
        });
        addLog('SUCCESS', `Rust Core trả về thành công ${data?.results?.length || 0} kết quả (source: ${data?.source})`);
      } catch (ipcErr) {
        addLog('WARN', `Tauri IPC gặp lỗi: ${ipcErr}. Chuyển sang Tier 2 (Direct HTTPS)...`);
      }
    }

    // Tier 2: Try Direct Web Fetch (Supports CORS)
    if (!data || !data.results || !data.results.length) {
      const isVn = (nat || '').toLowerCase().includes('vn');
      if (isVn && mode !== 'api') {
        data = generateClientMockUsers(count, gender, 'VN');
      } else {
        try {
          let apiUrl = `https://randomuser.me/api/?results=${count}`;
          if (gender) apiUrl += `&gender=${gender}`;
          if (nat && nat !== 'all') apiUrl += `&nat=${nat}`;

          addLog('INFO', `Đang gọi trực tiếp API randomuser.me: ${apiUrl}`);
          const controller = new AbortController();
          const timeoutId = setTimeout(() => controller.abort(), 4000);

          const res = await fetch(apiUrl, { signal: controller.signal });
          clearTimeout(timeoutId);

          if (res.ok) {
            data = await res.json();
            data.source = 'randomuser.me (Direct)';
            addLog('SUCCESS', `API trả về ${data.results.length} profiles qua HTTPS trực tiếp`);
          } else {
            addLog('WARN', `API trả về HTTP status: ${res.status}`);
          }
        } catch (netErr) {
          addLog('WARN', `Lỗi kết nối mạng: ${netErr.message}. Kích hoạt Tier 3 (Offline Emergency Engine)...`);
        }
      }
    }

    // Tier 3: Ultimate Fallback (Guaranteed to always work offline)
    if (!data || !data.results || !data.results.length) {
      data = generateClientMockUsers(count, gender, nat || 'US');
    }

    const endTime = performance.now();
    currentUsers = data.results || [];
    filteredUsers = [...currentUsers];

    // Update Stats
    const elapsed = Math.round(endTime - startTime);
    if (loadTime) loadTime.textContent = `${elapsed} ms`;
    if (resultCount) resultCount.textContent = `${currentUsers.length} profiles`;
    if (sourceInfo) {
      sourceInfo.textContent = data.source || (data.offlineFallback ? 'Offline' : 'API');
      if (data.offlineFallback || (data.source && data.source.includes('offline'))) {
        sourceInfo.className = 'px-2 py-0.5 rounded bg-amber-100 text-amber-800 font-mono text-[11px]';
      } else {
        sourceInfo.className = 'px-2 py-0.5 rounded bg-blue-100 text-blue-800 font-mono text-[11px]';
      }
    }

    renderData();
    if (loadingState) loadingState.classList.add('hidden');
    setTab(currentTab);
    addLog('SUCCESS', `Hoàn tất hiển thị ${currentUsers.length} profiles trong ${elapsed}ms.`);
  }
  window.fetchUsers = fetchUsers;

  // Render Data
  function renderData() {
    renderCards(filteredUsers);
    renderTable(filteredUsers);
    renderJSON(filteredUsers);
  }

  // Render Cards View
  function renderCards(users) {
    if (!cardsContainer) return;
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
            <div class="flex items-start space-x-3.5 pb-4 border-b border-slate-100">
              <img src="${avatar}" alt="${name}" class="w-14 h-14 rounded-2xl object-cover ring-2 ring-slate-100 shadow-sm" onerror="this.src='https://via.placeholder.com/150'">
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

            <div class="py-3 space-y-2 text-xs">
              <div class="copyable-field p-2 rounded-lg flex items-center justify-between group" onclick="window.copyToClipboard('${email}', 'Email')">
                <div class="flex items-center space-x-2 truncate">
                  <i class="fa-regular fa-envelope text-slate-400 w-4"></i>
                  <span class="text-slate-700 truncate">${email}</span>
                </div>
                <i class="fa-regular fa-copy text-slate-300 group-hover:text-blue-600 ml-2"></i>
              </div>

              <div class="copyable-field p-2 rounded-lg flex items-center justify-between group" onclick="window.copyToClipboard('${phone}', 'Số điện thoại')">
                <div class="flex items-center space-x-2 truncate">
                  <i class="fa-solid fa-phone text-slate-400 w-4"></i>
                  <span class="text-slate-700 truncate">${phone}</span>
                </div>
                <i class="fa-regular fa-copy text-slate-300 group-hover:text-blue-600 ml-2"></i>
              </div>

              <div class="copyable-field p-2 rounded-lg flex items-center justify-between group" onclick="window.copyToClipboard('${street}, ${cityState}, ${country}', 'Địa chỉ')">
                <div class="flex items-center space-x-2 truncate">
                  <i class="fa-solid fa-house text-slate-400 w-4"></i>
                  <span class="text-slate-700 truncate">${street || 'N/A'}</span>
                </div>
                <i class="fa-regular fa-copy text-slate-300 group-hover:text-blue-600 ml-2"></i>
              </div>

              <div class="bg-slate-50 p-2.5 rounded-xl border border-slate-100 flex items-center justify-between mt-2 font-mono text-[11px]">
                <div class="truncate mr-2">
                  <span class="text-slate-400">User:</span> <strong class="text-slate-800">${username}</strong>
                  <br>
                  <span class="text-slate-400">Pass:</span> <span class="text-emerald-700 font-bold">${password}</span>
                </div>
                <button onclick="window.copyToClipboard('${password}', 'Mật khẩu')" title="Copy Password" class="px-2 py-1 rounded bg-white border border-slate-200 hover:bg-slate-100 text-slate-600 shadow-2xs">
                  <i class="fa-solid fa-key"></i>
                </button>
              </div>
            </div>
          </div>

          <div class="pt-2 border-t border-slate-100 flex items-center justify-between text-[11px] text-slate-400">
            <span>Nat: <strong class="text-slate-600">${u.nat || 'Global'}</strong></span>
            <button onclick="window.copyToClipboard(JSON.stringify(window.currentUsers[${idx}], null, 2), 'JSON của profile')" class="hover:text-blue-600 transition">
              <i class="fa-solid fa-code"></i> Copy Object
            </button>
          </div>
        </div>
      `;
    }).join('');
  }

  // Render Table View
  function renderTable(users) {
    if (!tableBody) return;
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
            <img src="${avatar}" alt="" class="w-8 h-8 rounded-full object-cover border border-slate-200" onerror="this.src='https://via.placeholder.com/40'">
          </td>
          <td class="px-4 py-2.5 font-semibold text-slate-900">${name}</td>
          <td class="px-4 py-2.5 capitalize">${u.gender}</td>
          <td class="px-4 py-2.5 font-mono text-slate-600">
            <span class="cursor-pointer hover:underline" onclick="window.copyToClipboard('${u.email}', 'Email')">${u.email}</span>
          </td>
          <td class="px-4 py-2.5 font-mono">${u.phone}</td>
          <td class="px-4 py-2.5 truncate max-w-xs" title="${loc}">${loc}</td>
          <td class="px-4 py-2.5 font-mono text-slate-700">${u.login?.username || ''}</td>
          <td class="px-4 py-2.5 font-mono text-emerald-700 font-bold">
            <span class="cursor-pointer hover:underline" onclick="window.copyToClipboard('${u.login?.password}', 'Mật khẩu')">${u.login?.password || ''}</span>
          </td>
          <td class="px-4 py-2.5 text-right">
            <button class="px-2 py-1 rounded bg-slate-100 hover:bg-slate-200 text-slate-700" onclick="window.copyToClipboard(JSON.stringify(window.currentUsers[${idx}], null, 2), 'JSON')">
              <i class="fa-regular fa-copy"></i>
            </button>
          </td>
        </tr>
      `;
    }).join('');
  }

  // Render JSON
  function renderJSON(users) {
    if (rawJsonBlock) rawJsonBlock.textContent = JSON.stringify(users, null, 2);
  }

  // Format CSV String with UTF-8 BOM
  function generateCSVString(users) {
    if (!users.length) return '';
    const headers = [
      'Title', 'First Name', 'Last Name', 'Gender', 'Email',
      'Phone', 'Cell', 'Street', 'City', 'State', 'Country',
      'Postcode', 'Age', 'DOB', 'Username', 'Password', 'UUID', 'Nationality'
    ];

    const escapeCSV = (val) => {
      if (val === null || val === undefined) return '""';
      const str = String(val).replace(/"/g, '""');
      return `"${str}"`;
    };

    const rows = users.map((u) => {
      const street = u.location?.street
        ? `${u.location.street.number || ''} ${u.location.street.name || ''}`.trim()
        : '';

      return [
        escapeCSV(u.name?.title || ''),
        escapeCSV(u.name?.first || ''),
        escapeCSV(u.name?.last || ''),
        escapeCSV(u.gender || ''),
        escapeCSV(u.email || ''),
        escapeCSV(u.phone || ''),
        escapeCSV(u.cell || ''),
        escapeCSV(street),
        escapeCSV(u.location?.city || ''),
        escapeCSV(u.location?.state || ''),
        escapeCSV(u.location?.country || ''),
        escapeCSV(u.location?.postcode || ''),
        escapeCSV(u.dob?.age || ''),
        escapeCSV(u.dob?.date ? u.dob.date.split('T')[0] : ''),
        escapeCSV(u.login?.username || ''),
        escapeCSV(u.login?.password || ''),
        escapeCSV(u.login?.uuid || ''),
        escapeCSV(u.nat || '')
      ].join(',');
    });

    return '\uFEFF' + [headers.map(escapeCSV).join(','), ...rows].join('\r\n');
  }

  // Export to CSV
  async function exportCSV() {
    if (!currentUsers.length) {
      showToast('Chưa có dữ liệu để xuất!');
      return;
    }

    const csvContent = generateCSVString(currentUsers);
    const defaultName = `random-users-${Date.now()}.csv`;
    const invoke = getTauriInvoke();

    if (invoke) {
      try {
        addLog('INFO', 'Mở hộp thoại Windows Save File As (CSV)...');
        const savedPath = await invoke('save_file_dialog', {
          defaultName: defaultName,
          content: csvContent,
          extension: 'csv'
        });
        if (savedPath) {
          showToast(`Đã lưu file thành công!`);
          addLog('SUCCESS', `Đã lưu file CSV tại: ${savedPath}`);
          return;
        } else {
          addLog('INFO', 'Người dùng đã hủy lưu file.');
          return;
        }
      } catch (err) {
        addLog('WARN', `Tauri save dialog gặp lỗi: ${err}. Dùng fallback browser download.`);
      }
    }

    // Browser blob download fallback
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = defaultName;
    document.body.appendChild(a);
    a.click();
    a.remove();
    showToast('Đã tải xuống file CSV!');
    addLog('SUCCESS', `Đã tải xuống file CSV: ${defaultName}`);
  }
  window.exportCSV = exportCSV;

  // Export to JSON
  async function exportJSON() {
    if (!currentUsers.length) {
      showToast('Chưa có dữ liệu để xuất!');
      return;
    }

    const jsonContent = JSON.stringify(currentUsers, null, 2);
    const defaultName = `random-users-${Date.now()}.json`;
    const invoke = getTauriInvoke();

    if (invoke) {
      try {
        addLog('INFO', 'Mở hộp thoại Windows Save File As (JSON)...');
        const savedPath = await invoke('save_file_dialog', {
          defaultName: defaultName,
          content: jsonContent,
          extension: 'json'
        });
        if (savedPath) {
          showToast(`Đã lưu file thành công!`);
          addLog('SUCCESS', `Đã lưu file JSON tại: ${savedPath}`);
          return;
        } else {
          addLog('INFO', 'Người dùng đã hủy lưu file.');
          return;
        }
      } catch (err) {
        addLog('WARN', `Tauri save dialog gặp lỗi: ${err}. Dùng fallback browser download.`);
      }
    }

    // Browser blob download fallback
    const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(jsonContent);
    const a = document.createElement('a');
    a.href = dataStr;
    a.download = defaultName;
    document.body.appendChild(a);
    a.click();
    a.remove();
    showToast('Đã tải xuống file JSON!');
    addLog('SUCCESS', `Đã tải xuống file JSON: ${defaultName}`);
  }
  window.exportJSON = exportJSON;

  // Client-side quick filter
  if (filterInput) {
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
  }

  // Event Listeners
  if (btnGenerate) {
    btnGenerate.addEventListener('click', () => {
      addLog('INFO', "Nhấn nút 'Tạo Dữ Liệu Mới'");
      fetchUsers();
    });
  }

  if (btnExportCSV) btnExportCSV.addEventListener('click', exportCSV);
  if (btnExportJSON) btnExportJSON.addEventListener('click', exportJSON);
  if (btnCopyJSON) {
    btnCopyJSON.addEventListener('click', () => {
      copyToClipboard(JSON.stringify(currentUsers, null, 2), 'toàn bộ JSON');
    });
  }

  if (tabCards) tabCards.addEventListener('click', () => setTab('cards'));
  if (tabTable) tabTable.addEventListener('click', () => setTab('table'));
  if (tabJSON) tabJSON.addEventListener('click', () => setTab('json'));

  // Quick count buttons
  document.querySelectorAll('.quick-count').forEach(btn => {
    btn.addEventListener('click', () => {
      if (inputCount) inputCount.value = btn.dataset.val;
      document.querySelectorAll('.quick-count').forEach(b => {
        b.classList.remove('bg-blue-100', 'text-blue-700', 'font-bold');
        b.classList.add('bg-slate-100', 'text-slate-700');
      });
      btn.classList.add('bg-blue-100', 'text-blue-700', 'font-bold');
      btn.classList.remove('bg-slate-100', 'text-slate-700');
      addLog('INFO', `Chọn nhanh số lượng: ${btn.dataset.val}`);
      fetchUsers();
    });
  });

  // Debug Log Controls
  if (btnClearLog) {
    btnClearLog.addEventListener('click', () => {
      if (logContent) logContent.innerHTML = '';
      logHistory = [];
      logCount = 0;
      if (logCounter) logCounter.textContent = '0 sự kiện';
      addLog('INFO', 'Đã xóa nhật ký');
    });
  }

  if (btnCopyLog) {
    btnCopyLog.addEventListener('click', () => {
      copyToClipboard(logHistory.join('\n'), 'toàn bộ nhật ký');
    });
  }

  if (btnToggleLog) {
    btnToggleLog.addEventListener('click', () => {
      if (logContent) {
        logContent.classList.toggle('hidden');
        btnToggleLog.textContent = logContent.classList.contains('hidden') ? 'Mở rộng' : 'Thu gọn';
      }
    });
  }

  // Safe Initialization
  function init() {
    addLog('INFO', 'Hệ thống đã khởi tạo hoàn tất - Bắt đầu tải dữ liệu mẫu ban đầu...');
    fetchUsers();
  }

  if (document.readyState === 'loading') {
    window.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }

})();
