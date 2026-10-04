import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { UserProfile } from '../types/user';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export async function copyToClipboard(text: string): Promise<boolean> {
  try {
    if (navigator.clipboard && window.isSecureContext) {
      await navigator.clipboard.writeText(text);
      return true;
    } else {
      const textArea = document.createElement('textarea');
      textArea.value = text;
      textArea.style.position = 'fixed';
      textArea.style.left = '-999999px';
      textArea.style.top = '-999999px';
      document.body.appendChild(textArea);
      textArea.focus();
      textArea.select();
      const successful = document.execCommand('copy');
      textArea.remove();
      return successful;
    }
  } catch (err) {
    console.error('Failed to copy to clipboard:', err);
    return false;
  }
}

export function generateCSV(users: UserProfile[]): string {
  const headers = [
    'STT',
    'Ho Ten',
    'Chuc Danh',
    'Gioi Tinh',
    'Quoc Tich',
    'Email',
    'So Dien Thoai',
    'Avatar URL',
    'Dia Chi',
    'Thanh Pho',
    'Quoc Gia',
    'CCCD/SSN',
    'Username',
    'Password',
  ];

  const rows = users.map((u, i) => {
    const fullName = `${u.name?.title ? u.name.title + ' ' : ''}${u.name?.first || ''} ${u.name?.last || ''}`.trim();
    const job = u.job || '';
    const avatarUrl = u.picture?.large || u.picture?.medium || '';
    const street = `${u.location?.street?.number ? u.location.street.number + ' ' : ''}${u.location?.street?.name || ''}`.trim();
    const city = u.location?.city || '';
    const country = u.location?.country || '';
    const idVal = u.id?.value || '';

    const escapeCsv = (str: string | number) => `"${String(str || '').replace(/"/g, '""')}"`;

    return [
      i + 1,
      escapeCsv(fullName),
      escapeCsv(job),
      escapeCsv(u.gender || ''),
      escapeCsv(u.nat || ''),
      escapeCsv(u.email || ''),
      escapeCsv(u.phone || ''),
      escapeCsv(avatarUrl),
      escapeCsv(street),
      escapeCsv(city),
      escapeCsv(country),
      escapeCsv(idVal),
      escapeCsv(u.login?.username || ''),
      escapeCsv(u.login?.password || ''),
    ].join(',');
  });

  return [headers.join(','), ...rows].join('\r\n');
}

export function downloadBrowserFile(content: string, filename: string, mimeType: string) {
  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}
