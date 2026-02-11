import { writable, derived } from 'svelte/store';

export interface MenuItem {
    id: number;
    name: string;
    nameEn: string;
    price: number;
    category: string;
    image: string;
    description?: string;
}

export interface CartItem {
    item: MenuItem;
    qty: number;
}

export interface User {
    id: string;
    name: string;
    role: 'admin' | 'cashier';
}

function createCartStore() {
    const { subscribe, set, update } = writable<CartItem[]>([]);

    return {
        subscribe,
        add: (item: MenuItem) => {
            update(cart => {
                const existing = cart.find(c => c.item.id === item.id);
                if (existing) {
                    return cart.map(c =>
                        c.item.id === item.id
                            ? { ...c, qty: c.qty + 1 }
                            : c
                    );
                }
                return [...cart, { item, qty: 1 }];
            });
        },
        remove: (itemId: number) => {
            update(cart => cart.filter(c => c.item.id !== itemId));
        },
        updateQty: (itemId: number, qty: number) => {
            if (qty <= 0) {
                update(cart => cart.filter(c => c.item.id !== itemId));
            } else {
                update(cart => cart.map(c =>
                    c.item.id === itemId
                        ? { ...c, qty }
                        : c
                ));
            }
        },
        increment: (itemId: number) => {
            update(cart => cart.map(c =>
                c.item.id === itemId
                    ? { ...c, qty: c.qty + 1 }
                    : c
            ));
        },
        decrement: (itemId: number) => {
            update(cart => {
                const existing = cart.find(c => c.item.id === itemId);
                if (existing && existing.qty <= 1) {
                    return cart.filter(c => c.item.id !== itemId);
                }
                return cart.map(c =>
                    c.item.id === itemId
                        ? { ...c, qty: c.qty - 1 }
                        : c
                );
            });
        },
        clear: () => set([])
    };
}

export const cart = createCartStore();

export const cartTotal = derived(cart, $cart =>
    $cart.reduce((sum, c) => sum + c.item.price * c.qty, 0)
);

export const cartItemCount = derived(cart, $cart =>
    $cart.reduce((sum, c) => sum + c.qty, 0)
);

function createUserStore() {
    const { subscribe, set } = writable<User | null>(null);

    return {
        subscribe,
        login: (user: User) => set(user),
        logout: () => set(null)
    };
}

export const currentUser = createUserStore();

export const selectedCategory = writable<string>('coffee');

export type PaymentMethod = 'cash' | 'promptpay' | 'card';

export const selectedPaymentMethod = writable<PaymentMethod>('cash');
