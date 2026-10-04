import type { FetchUsersParams, FetchUsersResponse, UserProfile } from '../types/user';

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
    __TAURI__?: {
      core: {
        invoke: <T = unknown>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
      };
    };
  }
}

export function isTauri(): boolean {
  return typeof window !== 'undefined' && (Boolean(window.__TAURI_INTERNALS__) || Boolean(window.__TAURI__));
}

async function invokeTauri<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (window.__TAURI__?.core?.invoke) {
    return window.__TAURI__.core.invoke<T>(cmd, args);
  }
  // Dynamic import fallback for Tauri API
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<T>(cmd, args);
  } catch (err) {
    throw new Error(`Tauri environment not available for command "${cmd}"`);
  }
}

export async function fetchUsersApi(params: FetchUsersParams): Promise<FetchUsersResponse> {
  const mode = params.mode || 'local';
  const avatarStyle = params.avatarStyle || 'real';

  if (isTauri()) {
    return invokeTauri<FetchUsersResponse>('fetch_users', {
      count: params.count,
      gender: params.gender || null,
      nat: params.nat || null,
      mode,
      avatarStyle,
    });
  }

  // Browser Fallback (Vite Dev Server)
  if (mode === 'api') {
    let url = `https://randomuser.me/api/?results=${params.count}`;
    if (params.gender && params.gender !== 'all') {
      url += `&gender=${params.gender}`;
    }
    if (params.nat && params.nat !== 'all' && params.nat !== 'vn') {
      url += `&nat=${params.nat}`;
    }

    try {
      const res = await fetch(url);
      if (!res.ok) throw new Error(`HTTP error ${res.status}`);
      const data = await res.json();
      let results: UserProfile[] = data.results || [];
      if (avatarStyle === 'svg') {
        results = results.map((u) => ({
          ...u,
          picture: {
            large: `https://api.dicebear.com/7.x/avataaars/svg?seed=${u.login?.uuid || u.email}`,
            medium: `https://api.dicebear.com/7.x/avataaars/svg?seed=${u.login?.uuid || u.email}`,
            thumbnail: `https://api.dicebear.com/7.x/avataaars/svg?seed=${u.login?.uuid || u.email}`,
          },
        }));
      }
      return {
        results,
        source: 'RandomUser.me Web API',
        offlineFallback: false,
      };
    } catch (err: any) {
      console.warn('Browser direct fetch failed, generating client fallback:', err.message);
    }
  }

  // Local Offline Fallback with Real Photos or SVG
  const fallbackResults: UserProfile[] = Array.from({ length: params.count }).map((_, i) => {
    const isFemale = params.gender === 'female' ? true : params.gender === 'male' ? false : i % 2 === 1;
    const gender = isFemale ? 'female' : 'male';
    const photoId = (i * 7 + 12) % 99;
    const genderDir = isFemale ? 'women' : 'men';

    const picture = avatarStyle === 'svg'
      ? {
          large: `https://api.dicebear.com/7.x/avataaars/svg?seed=user_${i + 1}`,
          medium: `https://api.dicebear.com/7.x/avataaars/svg?seed=user_${i + 1}`,
          thumbnail: `https://api.dicebear.com/7.x/avataaars/svg?seed=user_${i + 1}`,
        }
      : {
          large: `https://randomuser.me/api/portraits/${genderDir}/${photoId}.jpg`,
          medium: `https://randomuser.me/api/portraits/med/${genderDir}/${photoId}.jpg`,
          thumbnail: `https://randomuser.me/api/portraits/thumb/${genderDir}/${photoId}.jpg`,
        };

    return {
      gender,
      name: { title: isFemale ? 'Ms' : 'Mr', first: isFemale ? 'Linh' : 'Tuấn', last: `Nguyễn ${i + 1}` },
      job: isFemale ? 'Thiết kế Giao diện (UI/UX)' : 'Kỹ sư Phần mềm (Senior)',
      location: {
        street: { number: 100 + i, name: 'Đường Nguyễn Huệ' },
        city: 'Hồ Chí Minh',
        state: 'TP.HCM',
        country: 'Vietnam',
        postcode: '70000',
      },
      email: `user${i + 1}@tuquet.io`,
      login: { username: `user_${i + 1}`, password: `Pass@${1000 + i}` },
      phone: `090${1000000 + i}`,
      cell: `098${1000000 + i}`,
      id: { name: 'CCCD', value: `079${100000000 + i}` },
      picture,
      nat: 'VN',
      dob: { date: '1995-05-15T00:00:00Z', age: 29 },
    };
  });

  return {
    results: fallbackResults,
    source: 'Offline Core (0ms)',
    offlineFallback: mode === 'api',
  };
}

export async function saveFileDialog(
  defaultName: string,
  content: string,
  extension: 'csv' | 'json'
): Promise<string | null> {
  if (isTauri()) {
    return invokeTauri<string | null>('save_file_dialog', {
      defaultName,
      content,
      extension,
    });
  }
  return null;
}

export async function downloadSingleAvatar(url: string, defaultName: string): Promise<string | null> {
  if (isTauri()) {
    return invokeTauri<string | null>('download_single_avatar', {
      url,
      defaultName,
    });
  }

  // Browser fallback
  try {
    const res = await fetch(url);
    const blob = await res.blob();
    const blobUrl = URL.createObjectURL(blob);
    const ext = url.includes('.svg') || url.includes('dicebear') ? 'svg' : 'jpg';
    const a = document.createElement('a');
    a.href = blobUrl;
    a.download = `${defaultName}.${ext}`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(blobUrl);
    return 'downloaded';
  } catch {
    window.open(url, '_blank');
    return 'opened';
  }
}

export async function exportBundleDialog(users: UserProfile[]): Promise<string | null> {
  if (isTauri()) {
    return invokeTauri<string | null>('export_bundle_dialog', {
      users,
    });
  }
  return null;
}

export async function logClientMessage(level: string, message: string): Promise<void> {
  if (isTauri()) {
    try {
      await invokeTauri('log_client_message', { level, message });
    } catch {
      // Ignore if not in Tauri
    }
  }
}

