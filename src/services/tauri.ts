import type { FetchUsersParams, FetchUsersResponse } from '../types/user';

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
  if (isTauri()) {
    return invokeTauri<FetchUsersResponse>('fetch_users', {
      count: params.count,
      gender: params.gender || null,
      nat: params.nat || null,
      mode: params.mode || 'auto',
    });
  }

  // Browser Fallback (Vite Dev Server)
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
    return {
      results: data.results || [],
      source: 'randomuser.me (web)',
      offlineFallback: false,
    };
  } catch (err: any) {
    console.warn('Browser direct fetch failed, generating client fallback:', err.message);
    // Minimal mock fallback for browser preview
    const fallbackResults = Array.from({ length: params.count }).map((_, i) => ({
      gender: params.gender === 'female' ? 'female' : 'male',
      name: { title: 'Mr', first: 'Nguyễn', last: `Văn ${i + 1}` },
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
      picture: {
        large: `https://api.dicebear.com/7.x/avataaars/svg?seed=user${i + 1}`,
        medium: `https://api.dicebear.com/7.x/avataaars/svg?seed=user${i + 1}`,
        thumbnail: `https://api.dicebear.com/7.x/avataaars/svg?seed=user${i + 1}`,
      },
      nat: 'VN',
      dob: { date: '1995-05-15T00:00:00Z', age: 29 },
    }));

    return {
      results: fallbackResults,
      source: 'web-local-fallback',
      offlineFallback: true,
      originalError: err.message,
    };
  }
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

export async function logClientMessage(level: string, message: string): Promise<void> {
  if (isTauri()) {
    try {
      await invokeTauri('log_client_message', { level, message });
    } catch {
      // Ignore if not in Tauri
    }
  }
}
