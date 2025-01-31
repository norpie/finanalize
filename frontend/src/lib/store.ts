import { writable, type Writable } from 'svelte/store';
import type User from '../models/user';

export const user: Writable<User | undefined> = writable(undefined);
