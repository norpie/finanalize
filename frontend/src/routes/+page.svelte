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

<div class="landing-page-wrapper min-h-screen bg-black text-white">
	<!-- Navbar -->
	<header class="flex items-center justify-between px-8 py-4">
		<div class="text-lg font-bold">
			<img src="/favicon.svg" alt="Logo" class="h-8" />
			<!-- Update the path to your logo -->
		</div>
		<nav class="flex gap-6">
			<a href="/about" class="hover:text-purple-400">About</a>
			<a href="/contact" class="hover:text-purple-400">Contact</a>
			<div class="relative">
				<button class="hover:text-purple-400">ENG ▼</button>
			</div>
		</nav>
		<div class="flex gap-2">
			<!-- Existing Button Logic -->
			<div class="button-wrapper mt-8 flex justify-center">
				{#if user}
					<Button onclick={() => goto('/dashboard')} class="bg-purple-700 hover:bg-purple-600"
						>Dashboard</Button
					>
				{:else if user === null}
					<Button onclick={() => goto('/login')} class="mr-2  bg-gray-600 hover:bg-purple-600"
						>Login</Button
					>

					<Button onclick={() => goto('/signup')} class="bg-purple-700 hover:bg-purple-600"
						>Sign Up</Button
					>
				{:else if user === undefined}
					<Button><Spinner color="purple" /></Button>
				{/if}
			</div>
		</div>
	</header>

	<!-- Hero Section -->
	<section class="hero py-16 text-center">
		<h1 class="mb-4 text-5xl font-bold">Easy. Financial. Expertise.</h1>
		<p class="mb-8 text-lg text-gray-300">
			Get easily accessible financial reports of your favorite companies.
		</p>

		<section class="features py-16 text-center">
			<div class="grid grid-cols-1 gap-6 md:grid-cols-3">
				<div
					class="feature-card rounded-lg bg-gray-800 p-6 text-white shadow transition-all duration-300"
				>
					<h3 class="mb-2 text-xl font-medium">Our Trained Finanalize Bot</h3>
					<p class="text-gray-400">Get insights from our advanced AI-driven bot.</p>
					<p class="mt-4 hidden text-gray-300">
						Our Finanalize Bot uses state-of-the-art AI algorithms to analyze financial data and provide
						you with actionable insights. Whether you're looking for trends, anomalies, or detailed
						reports, our bot has you covered.
					</p>
				</div>
				<div
					class="feature-card rounded-lg bg-gray-800 p-6 text-white shadow transition-all duration-300"
				>
					<h3 class="mb-2 text-xl font-medium">Prestigious Companies</h3>
					<p class="text-gray-400">Analyze data from the top companies worldwide.</p>
					<p class="mt-4 hidden text-gray-300">
						We provide detailed financial reports from the most prestigious companies around the
						globe. Stay informed with the latest data and make well-informed investment decisions.
					</p>
				</div>
				<div
					class="feature-card rounded-lg bg-gray-800 p-6 text-white shadow transition-all duration-300"
				>
					<h3 class="mb-2 text-xl font-medium">Precise Search</h3>
					<p class="text-gray-400">Quickly find the data you need with precision.</p>
					<p class="mt-4 hidden text-gray-300">
						Our precise search functionality allows you to quickly find the financial data you need.
						With advanced filtering and sorting options, you can easily navigate through vast
						amounts of information.
					</p>
				</div>
			</div>
		</section>
	</section>

	<!-- Footer Section -->
	<footer class="mt-16 bg-gray-900 py-6 text-center text-gray-400">
		<p>&copy; {new Date().getFullYear()} Finanalize. All rights reserved.</p>
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
