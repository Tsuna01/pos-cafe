<script lang="ts">
    import type { MenuItem } from '../stores';
    import { cart } from '../stores';

    export let item: MenuItem;

    function handleAdd() {
        cart.add(item);
    }
</script>

<button class="product-card" on:click={handleAdd}>
    <div class="card-image">
        <img src={item.image} alt={item.name} loading="lazy" />
    </div>
    <div class="card-content">
        <h3 class="card-title">{item.name}</h3>
        <p class="card-subtitle">{item.nameEn}</p>
        <div class="card-footer">
            <span class="price">{item.price}฿</span>
            <span class="add-icon">+</span>
        </div>
    </div>
</button>

<style>
    .product-card {
        background: var(--color-bg-card);
        border: 1px solid var(--color-bg-hover);
        border-radius: var(--radius-lg);
        overflow: hidden;
        cursor: pointer;
        transition: all var(--transition-normal);
        text-align: left;
        width: 100%;
        padding: 0;
        font-family: inherit;
    }

    .product-card:hover {
        border-color: var(--color-primary);
        transform: translateY(-4px);
        box-shadow: var(--shadow-lg), 0 0 20px rgba(196, 167, 125, 0.15);
    }

    .product-card:active {
        transform: translateY(-2px);
    }

    .card-image {
        width: 100%;
        height: 120px;
        overflow: hidden;
        background: var(--color-bg-tertiary);
    }

    .card-image img {
        width: 100%;
        height: 100%;
        object-fit: cover;
        transition: transform var(--transition-normal);
    }

    .product-card:hover .card-image img {
        transform: scale(1.05);
    }

    .card-content {
        padding: var(--space-3) var(--space-4);
    }

    .card-title {
        font-size: 0.9375rem;
        font-weight: 600;
        color: var(--color-text-primary);
        margin: 0 0 var(--space-1) 0;
        line-height: 1.3;
    }

    .card-subtitle {
        font-size: 0.75rem;
        color: var(--color-text-muted);
        margin: 0 0 var(--space-2) 0;
    }

    .card-footer {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .price {
        font-size: 1rem;
        font-weight: 700;
        color: var(--color-primary);
    }

    .add-icon {
        width: 28px;
        height: 28px;
        display: flex;
        align-items: center;
        justify-content: center;
        background: var(--color-primary);
        color: var(--color-bg-primary);
        border-radius: var(--radius-md);
        font-size: 1.25rem;
        font-weight: 600;
        transition: all var(--transition-fast);
    }

    .product-card:hover .add-icon {
        background: var(--color-primary-light);
        transform: scale(1.1);
    }
</style>
