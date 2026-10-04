/**
 * Local Random User Generator
 * Generates user profiles matching the randomuser.me data structure.
 * Supports offline operation using @faker-js/faker.
 */
let fakerInstance = null;

try {
  const { faker, fakerVI, fakerEN, fakerFR, fakerDE, fakerJA } = require('@faker-js/faker');
  fakerInstance = { faker, fakerVI, fakerEN, fakerFR, fakerDE, fakerJA };
} catch (err) {
  // Will be available after npm install completes
}

function getFakerForNat(nat) {
  if (!fakerInstance) return null;
  const n = (nat || '').toUpperCase();
  switch (n) {
    case 'VN':
      return fakerInstance.fakerVI || fakerInstance.faker;
    case 'FR':
      return fakerInstance.fakerFR || fakerInstance.faker;
    case 'DE':
      return fakerInstance.fakerDE || fakerInstance.faker;
    case 'JP':
      return fakerInstance.fakerJA || fakerInstance.faker;
    case 'US':
    case 'GB':
    case 'AU':
    case 'CA':
      return fakerInstance.fakerEN || fakerInstance.faker;
    default:
      return fakerInstance.faker;
  }
}

function generateLocalUser(options = {}) {
  const { gender, nat = 'US' } = options;
  const f = getFakerForNat(nat) || (fakerInstance ? fakerInstance.faker : null);

  const selectedGender = gender && ['male', 'female'].includes(gender.toLowerCase())
    ? gender.toLowerCase()
    : (Math.random() > 0.5 ? 'male' : 'female');

  if (f) {
    const sex = selectedGender;
    const firstName = f.person.firstName(sex);
    const lastName = f.person.lastName();
    const title = sex === 'male' ? 'Mr' : (Math.random() > 0.5 ? 'Ms' : 'Mrs');
    const birthDate = f.date.birthdate({ min: 18, max: 65, mode: 'age' });
    const age = new Date().getFullYear() - birthDate.getFullYear();
    const regDate = f.date.past({ years: 10 });
    const regAge = new Date().getFullYear() - regDate.getFullYear();

    const username = f.internet.username({ firstName, lastName }).toLowerCase().replace(/[^a-z0-9_]/g, '');
    const email = `${username}@example.com`;
    const avatarGender = sex === 'male' ? 'men' : 'women';
    const avatarId = Math.floor(Math.random() * 99) + 1;

    return {
      gender: sex,
      name: {
        title,
        first: firstName,
        last: lastName
      },
      location: {
        street: {
          number: f.number.int({ min: 10, max: 9999 }),
          name: f.location.street()
        },
        city: f.location.city(),
        state: f.location.state(),
        country: nat.toUpperCase() === 'VN' ? 'Vietnam' : f.location.country(),
        postcode: f.location.zipCode(),
        coordinates: {
          latitude: f.location.latitude().toString(),
          longitude: f.location.longitude().toString()
        },
        timezone: {
          offset: "+00:00",
          description: "UTC"
        }
      },
      email,
      login: {
        uuid: f.string.uuid(),
        username,
        password: f.internet.password({ length: 12 }),
        salt: f.string.alphanumeric(8),
        md5: f.string.hexadecimal({ length: 32, prefix: '' }).toLowerCase(),
        sha1: f.string.hexadecimal({ length: 40, prefix: '' }).toLowerCase(),
        sha256: f.string.hexadecimal({ length: 64, prefix: '' }).toLowerCase()
      },
      dob: {
        date: birthDate.toISOString(),
        age
      },
      registered: {
        date: regDate.toISOString(),
        age: regAge
      },
      phone: f.phone.number(),
      cell: f.phone.number(),
      id: {
        name: nat.toUpperCase(),
        value: f.string.numeric(9)
      },
      picture: {
        large: `https://randomuser.me/api/portraits/${avatarGender}/${avatarId}.jpg`,
        medium: `https://randomuser.me/api/portraits/med/${avatarGender}/${avatarId}.jpg`,
        thumbnail: `https://randomuser.me/api/portraits/thumb/${avatarGender}/${avatarId}.jpg`
      },
      nat: nat.toUpperCase()
    };
  }

  // Pure fallback if faker is still initializing
  const isMale = selectedGender === 'male';
  const first = isMale ? 'Alex' : 'Emma';
  const last = 'Smith';
  const randomNum = Math.floor(Math.random() * 900) + 100;
  return {
    gender: selectedGender,
    name: { title: isMale ? 'Mr' : 'Ms', first, last },
    location: {
      street: { number: 123, name: 'Main Street' },
      city: 'Sample City',
      state: 'Sample State',
      country: 'United States',
      postcode: '10001',
      coordinates: { latitude: '0.000', longitude: '0.000' },
      timezone: { offset: '+00:00', description: 'UTC' }
    },
    email: `${first.toLowerCase()}.${last.toLowerCase()}${randomNum}@example.com`,
    login: {
      uuid: 'mock-uuid-' + randomNum,
      username: `${first.toLowerCase()}${randomNum}`,
      password: 'Pass_' + Math.random().toString(36).slice(-8) + '!'
    },
    dob: { date: '1995-05-15T00:00:00.000Z', age: 29 },
    registered: { date: '2020-01-01T00:00:00.000Z', age: 4 },
    phone: `(555) 019-${randomNum}`,
    cell: `(555) 018-${randomNum}`,
    id: { name: 'ID', value: `${randomNum}888` },
    picture: {
      large: `https://randomuser.me/api/portraits/${isMale ? 'men' : 'women'}/1.jpg`,
      medium: `https://randomuser.me/api/portraits/med/${isMale ? 'men' : 'women'}/1.jpg`,
      thumbnail: `https://randomuser.me/api/portraits/thumb/${isMale ? 'men' : 'women'}/1.jpg`
    },
    nat: nat.toUpperCase()
  };
}

function generateLocalUsers(count = 10, options = {}) {
  const results = [];
  for (let i = 0; i < count; i++) {
    results.push(generateLocalUser(options));
  }
  return {
    results,
    info: {
      seed: 'offline-local-generator',
      results: count,
      page: 1,
      version: '1.4-local'
    }
  };
}

module.exports = {
  generateLocalUser,
  generateLocalUsers
};
