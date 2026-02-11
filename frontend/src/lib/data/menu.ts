import type { MenuItem } from '../stores';

let invoke: any = null;

async function getInvoke() {
    if (invoke) return invoke;
    try {
        const tauri = await import('@tauri-apps/api/tauri');
        invoke = tauri.invoke;
        return invoke;
    } catch {
        return null;
    }
}


export interface Category {
    id: string;
    name: string;
    icon: string;
}


interface DbMenuItem {
    id: number;
    name: string;
    name_en: string;
    price: number;
    category_id: string;
    image: string;
    description: string;
    is_available: boolean;
}

function mapDbItem(item: DbMenuItem): MenuItem {
    return {
        id: item.id,
        name: item.name,
        nameEn: item.name_en,
        price: item.price,
        category: item.category_id,
        image: item.image,
        description: item.description,
    };
}


export async function fetchCategories(): Promise<Category[]> {
    const inv = await getInvoke();
    if (inv) {
        try {
            return await inv('get_categories');
        } catch (err) {
            console.error('fetchCategories failed:', err);
        }
    }
    return [];
}

export async function fetchMenuItems(category?: string): Promise<MenuItem[]> {
    const inv = await getInvoke();
    if (inv) {
        try {
            const dbItems: DbMenuItem[] = await inv('get_menu_items', {
                category: category ?? null,
            });
            return dbItems.map(mapDbItem);
        } catch (err) {
            console.error('fetchMenuItems failed:', err);
        }
    }
    return [];
}
