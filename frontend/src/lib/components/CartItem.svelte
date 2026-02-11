<script lang="ts">
    import type { CartItem as CartItemType } from "../stores";
    import { cart } from "../stores";

    let { entry }: { entry: CartItemType } = $props();

    function increment() {
        cart.increment(entry.item.id);
    }

    function decrement() {
        cart.decrement(entry.item.id);
    }

    function remove() {
        cart.remove(entry.item.id);
    }
</script>

<div class="cart-item">
    <div class="item-info">
        <span class="item-name">{entry.item.name}</span>
        <span class="item-price">{entry.item.price}฿ × {entry.qty}</span>
    </div>
    <div class="item-controls">
        <button class="qty-btn" onclick={decrement} aria-label="ลดจำนวน"
            >−</button
        >
        <span class="qty">{entry.qty}</span>
        <button class="qty-btn" onclick={increment} aria-label="เพิ่มจำนวน"
            >+</button
        >
        <button class="remove-btn" onclick={remove} aria-label="ลบ">✕</button>
    </div>
    <div class="item-total">
        {entry.item.price * entry.qty}฿
    </div>
</div>

<style>
    .cart-item {
        display: grid;
        grid-template-columns: 1fr auto auto;
        gap: var(--space-3);
        align-items: center;
        padding: var(--space-3);
        background: var(--color-bg-tertiary);
        border-radius: var(--radius-md);
        transition: background var(--transition-fast);
    }

    .cart-item:hover {
        background: var(--color-bg-hover);
    }

    .item-info {
        display: flex;
        flex-direction: column;
        gap: var(--space-1);
        min-width: 0;
    }

    .item-name {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--color-text-primary);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .item-price {
        font-size: 0.75rem;
        color: var(--color-text-muted);
    }

    .item-controls {
        display: flex;
        align-items: center;
        gap: var(--space-1);
    }

    .qty-btn {
        width: 28px;
        height: 28px;
        display: flex;
        align-items: center;
        justify-content: center;
        background: var(--color-bg-card);
        border: 1px solid var(--color-bg-hover);
        border-radius: var(--radius-sm);
        color: var(--color-text-primary);
        font-size: 1rem;
        cursor: pointer;
        transition: all var(--transition-fast);
    }

    .qty-btn:hover {
        background: var(--color-primary);
        border-color: var(--color-primary);
        color: var(--color-bg-primary);
    }

    .qty {
        min-width: 24px;
        text-align: center;
        font-weight: 600;
        font-size: 0.875rem;
    }

    .remove-btn {
        width: 28px;
        height: 28px;
        display: flex;
        align-items: center;
        justify-content: center;
        background: transparent;
        border: none;
        border-radius: var(--radius-sm);
        color: var(--color-text-muted);
        font-size: 0.875rem;
        cursor: pointer;
        transition: all var(--transition-fast);
        margin-left: var(--space-2);
    }

    .remove-btn:hover {
        background: var(--color-danger-bg);
        color: var(--color-danger);
    }

    .item-total {
        font-weight: 600;
        color: var(--color-primary);
        min-width: 60px;
        text-align: right;
    }
</style>
