export interface UserLocation {
  street?: {
    number?: number | string;
    name?: string;
  };
  city?: string;
  state?: string;
  country?: string;
  postcode?: string | number;
}

export interface UserName {
  title?: string;
  first: string;
  last: string;
}

export interface UserPicture {
  large: string;
  medium: string;
  thumbnail: string;
  data_uri?: string;
}

export interface UserLogin {
  uuid?: string;
  username: string;
  password?: string;
}

export interface UserProfile {
  gender: string;
  name: UserName;
  job?: string;
  id?: {
    name?: string;
    value?: string;
  };
  location: UserLocation;
  email: string;
  login: UserLogin;
  phone: string;
  cell?: string;
  picture: UserPicture;
  nat: string;
  dob?: {
    date: string;
    age: number;
  };
}

export interface FetchUsersParams {
  count: number;
  gender?: string;
  nat?: string;
  mode?: 'local' | 'api' | 'auto';
  avatarStyle?: 'real' | 'svg';
}

export interface FetchUsersResponse {
  results: UserProfile[];
  source?: string;
  offlineFallback?: boolean;
  originalError?: string;
}

export interface LogEntry {
  id: string;
  time: string;
  level: 'INFO' | 'SUCCESS' | 'WARN' | 'ERROR';
  message: string;
}
