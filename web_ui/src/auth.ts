import { createPromiseClient } from "@bufbuild/connect";
import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { AuthService } from "../gen/traceguard/v1/auth_connect";

const transport = createGrpcWebTransport({
  baseUrl: "http://localhost:8080",
});

const authClient = createPromiseClient(AuthService, transport);

export async function login(username: string, password: string): Promise<string> {
  const response = await authClient.login({ username, password });
  return response.token;
}

export function setAuthToken(token: string) {
  localStorage.setItem('authToken', token);
}

export function getAuthToken(): string | null {
  return localStorage.getItem('authToken');
}

export function removeAuthToken() {
  localStorage.removeItem('authToken');
}