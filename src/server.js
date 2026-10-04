const express = require('express');
const cors = require('cors');
const path = require('path');
const axios = require('axios');
const { generateLocalUsers } = require('./localGenerator');
const { usersToCSV } = require('./exporter');

const app = express();
const PORT = process.env.PORT || 3500;

app.use(cors());
app.use(express.json({ limit: '10mb' }));
app.use(express.static(path.join(__dirname, '../public')));

/**
 * GET /api/users
 * Query params:
 * - results: number (1 - 5000, default 10)
 * - gender: 'male' | 'female'
 * - nat: string (comma-separated, e.g. 'us,gb,vn,fr')
 * - mode: 'api' | 'local' | 'auto' (default 'auto')
 * - seed: string
 * - page: number
 */
app.get('/api/users', async (req, res) => {
  const results = Math.min(Math.max(parseInt(req.query.results, 10) || 10, 1), 5000);
  const gender = req.query.gender || '';
  const nat = req.query.nat || '';
  const mode = req.query.mode || 'auto';
  const seed = req.query.seed || '';
  const page = parseInt(req.query.page, 10) || 1;

  // If local mode is requested or nationality includes 'vn' (since randomuser.me has limited VN support)
  const isVnRequested = nat.toLowerCase().split(',').includes('vn');

  if (mode === 'local' || (isVnRequested && mode !== 'api')) {
    try {
      const data = generateLocalUsers(results, { gender, nat: isVnRequested ? 'VN' : (nat.split(',')[0] || 'US') });
      return res.json({
        ...data,
        source: 'local-offline',
        offlineFallback: false
      });
    } catch (err) {
      console.error('Local generator error:', err);
    }
  }

  // Attempt to fetch from official randomuser.me API
  try {
    const params = { results };
    if (gender) params.gender = gender;
    if (nat) params.nat = nat;
    if (seed) params.seed = seed;
    if (page) params.page = page;

    const response = await axios.get('https://randomuser.me/api/', {
      params,
      timeout: 5000,
      headers: {
        'User-Agent': 'RandomUserLocalTool/1.0'
      }
    });

    return res.json({
      ...response.data,
      source: 'randomuser.me',
      offlineFallback: false
    });
  } catch (err) {
    console.warn(`[RandomUser API Warning] ${err.message}. Switching to local offline generator.`);
    // Seamless fallback to local generator
    const fallbackData = generateLocalUsers(results, {
      gender,
      nat: nat ? nat.split(',')[0] : 'US'
    });

    return res.json({
      ...fallbackData,
      source: 'local-fallback',
      offlineFallback: true,
      originalError: err.message
    });
  }
});

/**
 * POST /api/export/csv
 * Converts user payload to downloadable CSV
 */
app.post('/api/export/csv', (req, res) => {
  const users = req.body.users || [];
  if (!Array.isArray(users) || users.length === 0) {
    return res.status(400).send('No user data provided');
  }

  const csvContent = usersToCSV(users);
  const filename = `random-users-${Date.now()}.csv`;

  res.setHeader('Content-Type', 'text/csv; charset=utf-8');
  res.setHeader('Content-Disposition', `attachment; filename="${filename}"`);
  return res.send(csvContent);
});

// Health check
app.get('/api/health', (req, res) => {
  res.json({ status: 'ok', uptime: process.uptime() });
});

// Fallback to index.html
app.get('*', (req, res) => {
  res.sendFile(path.join(__dirname, '../public/index.html'));
});

app.listen(PORT, () => {
  console.log(`====================================================`);
  console.log(`🚀 Random User Generator App is running!`);
  console.log(`🌐 Local URL: http://localhost:${PORT}`);
  console.log(`📁 Workspace: D:\\Repository\\tuquet\\random-user-generator`);
  console.log(`====================================================`);
});
