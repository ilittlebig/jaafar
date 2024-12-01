/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-01
 */

export interface Account {
  email: string;
  firstname: string;
  lastname: string;
  phone: string;
  address1: string;
  address2: string;
  city: string;
  postcode: string;
  country: string;
}

export let accountsStore: Account[] = $state([]);
