export interface AuthenticatePayload {
  username: string;
  password: string;
}

export interface ChangePasswordPayload {
  oldPassword: string;
  newPassword: string;
}

export interface SignupPayload {
  username: string;
  password: string;
}
