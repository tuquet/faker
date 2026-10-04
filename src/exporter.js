/**
 * Exporter utilities for Random User Generator
 * Converts JSON user objects to CSV, formatted JSON, etc.
 */

function usersToCSV(users = []) {
  if (!users.length) return '';

  const headers = [
    'Title',
    'First Name',
    'Last Name',
    'Gender',
    'Email',
    'Phone',
    'Cell',
    'Street',
    'City',
    'State',
    'Country',
    'Postcode',
    'Age',
    'DOB',
    'Username',
    'Password',
    'UUID',
    'Nationality',
    'Picture Large'
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
      escapeCSV(u.nat || ''),
      escapeCSV(u.picture?.large || '')
    ].join(',');
  });

  // Include UTF-8 BOM for Excel Vietnamese/Unicode support
  return '\uFEFF' + [headers.map(escapeCSV).join(','), ...rows].join('\r\n');
}

module.exports = {
  usersToCSV
};
