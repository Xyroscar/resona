import { writable } from 'svelte/store';
import type { Collection, Variable, Request as ApiRequest } from '../types';

export const collections = writable<Collection[]>([]);
export const variables = writable<Variable[]>([]);
export const activeCollection = writable<Collection | null>(null);
export const activeRequest = writable<ApiRequest | null>(null);
export const currentResponse = writable<{
  status: number;
  statusText: string;
  headers: Record<string, string>;
  body: string;
  time: number;
} | null>(null); 