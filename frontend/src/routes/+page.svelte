<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';
    import Spinner from '$lib/components/spinner.svelte';

    import { request } from '$lib/request';

    import { onMount } from 'svelte';
    import type User from '../models/user';
    import { goto } from '$app/navigation';

    let user: User | undefined | null = $state<User>();

    onMount(async () => {
        user = (await request<User>('GET', 'v1/protected/me', true)).result;
        if (user === undefined) {
            user = null;
        }
    });
</script>

<div class="landing-page-wrapper bg-black text-white min-h-screen">
    <!-- Navbar -->
    <header class="flex items-center justify-between px-8 py-4">
        <div class="text-lg font-bold">
            <img src="/favicon.svg" alt="Logo" class="h-8" /> <!-- Update the path to your logo -->
        </div>
        <nav class="flex gap-6">
            <a href="/about" class="hover:text-purple-400">About</a>
            <a href="/contact" class="hover:text-purple-400">Contact</a>
            <div class="relative">
                <button class="hover:text-purple-400">ENG ▼</button>
            </div>
        </nav>
        <div class="flex gap-2">
            <a href="/login">
                <Button class="bg-gray-700 hover:bg-gray-600">Log In</Button>
            </a>
            <a href="/register">
                <Button class="bg-purple-700 hover:bg-purple-600">Sign Up</Button>
            </a>
        </div>
    </header>

    <!-- Hero Section -->
    <section class="hero py-16 text-center">
        <h1 class="text-5xl font-bold mb-4">Easy. Financial. Expertise.</h1>
        <p class="text-lg text-gray-300 mb-8">
            Get easily accessible financial reports of your favorite companies.
        </p>
        
        <section class="features py-16 text-center">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="feature-card p-6 bg-gray-800 rounded-lg shadow text-white transition-all duration-300">
                    <h3 class="text-xl font-medium mb-2">Our Trained FinAn Bot</h3>
                    <p class="text-gray-400">Get insights from our advanced AI-driven bot.</p>
                    <p class="hidden text-gray-300 mt-4">Our FinAn Bot uses state-of-the-art AI algorithms to analyze financial data and provide you with actionable insights. Whether you're looking for trends, anomalies, or detailed reports, our bot has you covered.</p>
                </div>
                <div class="feature-card p-6 bg-gray-800 rounded-lg shadow text-white transition-all duration-300">
                    <h3 class="text-xl font-medium mb-2">Prestigious Companies</h3>
                    <p class="text-gray-400">Analyze data from the top companies worldwide.</p>
                    <p class="hidden text-gray-300 mt-4">We provide detailed financial reports from the most prestigious companies around the globe. Stay informed with the latest data and make well-informed investment decisions.</p>
                </div>
                <div class="feature-card p-6 bg-gray-800 rounded-lg shadow text-white transition-all duration-300">
                    <h3 class="text-xl font-medium mb-2">Precise Search</h3>
                    <p class="text-gray-400">Quickly find the data you need with precision.</p>
                    <p class="hidden text-gray-300 mt-4">Our precise search functionality allows you to quickly find the financial data you need. With advanced filtering and sorting options, you can easily navigate through vast amounts of information.</p>
                </div>
            </div>
        </section>
    </section>

    

    <!-- Existing Button Logic -->
    <div class="button-wrapper mt-8 flex justify-center">
        {#if user}
            <Button onclick={() => goto('/dashboard')} class="bg-purple-700 hover:bg-purple-600">Dashboard</Button>
        {:else if user === null}
            <Button onclick={() => goto('/login')} class="bg-purple-700 hover:bg-purple-600">Login</Button>
        {:else if user === undefined}
            <Button><Spinner color="purple" /></Button>
        {/if}
    </div>

    <!-- Footer Section -->
    <footer class="bg-gray-900 text-gray-400 py-6 text-center mt-16">
        <p>&copy; {new Date().getFullYear()} Our Company. All rights reserved.</p>
    </footer>
</div>

<style>
    .hero {
        background-image: linear-gradient(to right, #2d2d2d, #1a1a1a);
    }

    .feature-card:hover {
        transform: scale(1.05);
        transition: transform 0.2s ease-in-out;
        background-color: #4c3c88;
    }
</style>