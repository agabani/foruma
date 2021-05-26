export interface ChangePasswordPayload {
  currentPassword: string;
  newPassword: string;
}

export interface LoginPayload {
  username: string;
  password: string;
}

export interface SignupPayload {
  username: string;
  password: string;
}
