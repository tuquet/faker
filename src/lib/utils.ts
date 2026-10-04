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
    'Full Name',
    'First Name',
    'Last Name',
    'Gender',
    'Date of Birth',
    'Age',
    'CCCD / ID',
    'Email',
    'Username',
    'Password',
    'Phone',
    'Street Address',
    'Ward',
    'District',
    'City',
    'Postcode',
    'Country',
    'Job',
    'Avatar URL',
  ];

  const rows = users.map((u, i) => {
    const fullName = `${u.name?.title ? u.name.title + ' ' : ''}${u.name?.first || ''} ${u.name?.last || ''}`.trim();
    const firstName = u.name?.first || '';
    const lastName = u.name?.last || '';
    const dob = u.dob?.date ? u.dob.date.slice(0, 10) : '';
    const age = u.dob?.age || '';
    const idVal = u.id?.value || '';
    const street = `${u.location?.street?.number ? u.location.street.number + ' ' : ''}${u.location?.street?.name || ''}`.trim();
    const ward = u.location?.ward || '';
    const district = u.location?.district || '';
    const city = u.location?.city || '';
    const postcode = u.location?.postcode || '';
    const country = u.location?.country || '';
    const job = u.job || '';
    const avatarUrl = u.picture?.large || u.picture?.medium || '';

    const escapeCsv = (str: string | number) => `"${String(str || '').replace(/"/g, '""')}"`;

    return [
      i + 1,
      escapeCsv(fullName),
      escapeCsv(firstName),
      escapeCsv(lastName),
      escapeCsv(u.gender || ''),
      escapeCsv(dob),
      escapeCsv(age),
      escapeCsv(idVal),
      escapeCsv(u.email || ''),
      escapeCsv(u.login?.username || ''),
      escapeCsv(u.login?.password || ''),
      escapeCsv(u.phone || ''),
      escapeCsv(street),
      escapeCsv(ward),
      escapeCsv(district),
      escapeCsv(city),
      escapeCsv(postcode),
      escapeCsv(country),
      escapeCsv(job),
      escapeCsv(avatarUrl),
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
