import type { MenuItem } from '../stores';

export const categories = [
    { id: 'coffee', name: 'กาแฟ', icon: '☕' },
    { id: 'drinks', name: 'เครื่องดื่ม', icon: '🧋' },
    { id: 'bakery', name: 'เบเกอรี่', icon: '🥐' },
    { id: 'dessert', name: 'ของหวาน', icon: '🍰' }
];

// Using placeholder images from picsum.photos
const coffeeImg = 'https://images.unsplash.com/photo-1509042239860-f550ce710b93?w=200&h=200&fit=crop';
const latteImg = 'https://images.unsplash.com/photo-1534778101976-62847782c213?w=200&h=200&fit=crop';
const espressoImg = 'https://images.unsplash.com/photo-1510591509098-f4fdc6d0ff04?w=200&h=200&fit=crop';
const mochaImg = 'https://images.unsplash.com/photo-1578314675249-a6910f80cc4e?w=200&h=200&fit=crop';
const matchaImg = 'https://images.unsplash.com/photo-1515823064-d6e0c04616a7?w=200&h=200&fit=crop';
const smoothieImg = 'https://images.unsplash.com/photo-1505252585461-04db1eb84625?w=200&h=200&fit=crop';
const croissantImg = 'https://images.unsplash.com/photo-1555507036-ab1f4038808a?w=200&h=200&fit=crop';
const cakeImg = 'https://images.unsplash.com/photo-1578985545062-69928b1d9587?w=200&h=200&fit=crop';

export const menuItems: MenuItem[] = [
    // Coffee
    { id: 1, name: 'อเมริกาโน่', nameEn: 'Americano', price: 60, category: 'coffee', image: coffeeImg, description: 'เอสเพรสโซ่ผสมน้ำร้อน' },
    { id: 2, name: 'ลาเต้', nameEn: 'Latte', price: 70, category: 'coffee', image: latteImg, description: 'เอสเพรสโซ่ผสมนมนุ่ม' },
    { id: 3, name: 'คาปูชิโน่', nameEn: 'Cappuccino', price: 75, category: 'coffee', image: coffeeImg, description: 'เอสเพรสโซ่ นมร้อน ฟองนม' },
    { id: 4, name: 'เอสเพรสโซ่', nameEn: 'Espresso', price: 55, category: 'coffee', image: espressoImg, description: 'กาแฟเข้มข้น' },
    { id: 5, name: 'มอคค่า', nameEn: 'Mocha', price: 85, category: 'coffee', image: mochaImg, description: 'เอสเพรสโซ่ผสมช็อกโกแลต' },
    { id: 6, name: 'คาราเมล มัคคิอาโต้', nameEn: 'Caramel Macchiato', price: 90, category: 'coffee', image: latteImg, description: 'ลาเต้ราดคาราเมล' },
    { id: 7, name: 'อเมริกาโน่เย็น', nameEn: 'Iced Americano', price: 65, category: 'coffee', image: coffeeImg, description: 'อเมริกาโน่เย็นสดชื่น' },
    { id: 8, name: 'ลาเต้เย็น', nameEn: 'Iced Latte', price: 75, category: 'coffee', image: latteImg, description: 'ลาเต้เย็นครีมมี่' },

    // Drinks
    { id: 20, name: 'ชาเขียวมัทฉะ', nameEn: 'Matcha Latte', price: 80, category: 'drinks', image: matchaImg, description: 'มัทฉะผสมนมสด' },
    { id: 21, name: 'ชาไทย', nameEn: 'Thai Tea', price: 60, category: 'drinks', image: smoothieImg, description: 'ชาไทยหอมหวาน' },
    { id: 22, name: 'ช็อกโกแลต', nameEn: 'Hot Chocolate', price: 70, category: 'drinks', image: mochaImg, description: 'ช็อกโกแลตร้อนเข้มข้น' },
    { id: 23, name: 'สมูทตี้เบอร์รี่', nameEn: 'Berry Smoothie', price: 85, category: 'drinks', image: smoothieImg, description: 'ผลเบอร์รี่รวม' },
    { id: 24, name: 'สมูทตี้มะม่วง', nameEn: 'Mango Smoothie', price: 85, category: 'drinks', image: smoothieImg, description: 'มะม่วงสดปั่น' },
    { id: 25, name: 'น้ำส้มคั้นสด', nameEn: 'Fresh Orange Juice', price: 65, category: 'drinks', image: smoothieImg, description: 'ส้มคั้นสด 100%' },

    // Bakery
    { id: 40, name: 'ครัวซองต์', nameEn: 'Croissant', price: 55, category: 'bakery', image: croissantImg, description: 'ครัวซองต์เนยสด' },
    { id: 41, name: 'ครัวซองต์ช็อกโกแลต', nameEn: 'Pain au Chocolat', price: 65, category: 'bakery', image: croissantImg, description: 'ครัวซองต์ไส้ช็อกโกแลต' },
    { id: 42, name: 'มัฟฟินบลูเบอร์รี่', nameEn: 'Blueberry Muffin', price: 50, category: 'bakery', image: cakeImg, description: 'มัฟฟินไส้บลูเบอร์รี่' },
    { id: 43, name: 'บราวนี่', nameEn: 'Brownie', price: 55, category: 'bakery', image: cakeImg, description: 'บราวนี่ช็อกโกแลตเข้ม' },
    { id: 44, name: 'คุกกี้ช็อกชิพ', nameEn: 'Chocolate Chip Cookie', price: 35, category: 'bakery', image: cakeImg, description: 'คุกกี้นุ่มใส่ช็อกโกแลตชิพ' },

    // Dessert
    { id: 60, name: 'เค้กช็อกโกแลต', nameEn: 'Chocolate Cake', price: 85, category: 'dessert', image: cakeImg, description: 'เค้กช็อกโกแลตเข้มข้น' },
    { id: 61, name: 'ชีสเค้ก', nameEn: 'Cheesecake', price: 90, category: 'dessert', image: cakeImg, description: 'ชีสเค้กนิวยอร์ค' },
    { id: 62, name: 'ทีรามิสุ', nameEn: 'Tiramisu', price: 95, category: 'dessert', image: cakeImg, description: 'ทีรามิสุกาแฟสไตล์อิตาเลียน' },
    { id: 63, name: 'มาการอง (3 ชิ้น)', nameEn: 'Macarons (3 pcs)', price: 75, category: 'dessert', image: cakeImg, description: 'มาการองฝรั่งเศส' },
    { id: 64, name: 'ไอศกรีมถ้วย', nameEn: 'Ice Cream Cup', price: 60, category: 'dessert', image: cakeImg, description: 'ไอศกรีม 2 สกู๊ป' }
];

export function getMenuByCategory(category: string): MenuItem[] {
    return menuItems.filter(item => item.category === category);
}
