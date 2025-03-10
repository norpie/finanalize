<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import Spinner from '$lib/components/spinner.svelte';

	import { goto } from '$app/navigation';
	import { user } from '$lib/store';
	import { func } from 'effect/FastCheck';

	$inspect($user);

	function showText(event) {
		event.target.querySelectorAll('p')[1].classList.remove('hidden');
	}	
	function hideText(event) {
		event.target.querySelectorAll('p')[1].classList.add('hidden');
	}
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
		</nav>
		<div class="flex gap-2">
			<!-- Existing Button Logic -->
			<div class="button-wrapper mt-8 flex justify-center">
				{#if $user}
					<Button onclick={() => goto('/dashboard')} class="bg-purple-700 hover:bg-purple-600"
						>Dashboard</Button
					>
				{:else if $user === null}
					<Button onclick={() => goto('/login')} class="mr-2  bg-gray-600 hover:bg-purple-600"
						>Login</Button
					>

					<Button onclick={() => goto('/register')} class="bg-purple-700 hover:bg-purple-600"
						>Sign Up</Button
					>
				{:else if $user === undefined}
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
				role="contentinfo"
					class="feature-card rounded-lg bg-purple-900 p-6 text-white shadow transition-all duration-300"
					onmouseenter={showText} onmouseleave={hideText}
				>
					<h3 class="mb-2 text-xl font-medium">Your AI-Powered Investment Assistant</h3>
					<p class="text-gray-400">
						Make data-driven decisions with cutting-edge financial analysis.
					</p>
					<p class="mt-4 hidden text-gray-300">
						Our AI-driven assistant scours the internet, aggregating and analyzing financial data
						from trusted sources, news articles, company reports, and market trends. Using advanced
						data-scraping techniques, it compiles a well-structured financial report tailored to the
						company, stock, or asset you’re researching. Whether you need insights on historical
						performance, industry sentiment, or risk factors, our system ensures you have all the
						relevant data in one place to make informed investment decisions with confidence.
					</p>
				</div>
				<div
				role="contentinfo"
					class="feature-card rounded-lg bg-purple-900 p-6 text-white shadow transition-all duration-300"
					onmouseenter={showText} onmouseleave={hideText}
				>
					<h3 class="mb-2 text-xl font-medium">Prestigious Companies</h3>
					<p class="text-gray-400">Analyze data from the top companies worldwide.</p>
					<p class="mt-4 hidden text-gray-300">
						Stay informed with comprehensive financial reports from leading global companies across
						various industries. Our system aggregates and processes real-time market data,
						historical performance records, and industry insights, giving you a complete picture of
						a company's financial health. With this valuable information, you can confidently assess
						potential investment opportunities and align them with your financial goals.
					</p>
				</div>
				<div
				role="contentinfo"
					class="feature-card rounded-lg bg-purple-900 p-6 text-white shadow transition-all duration-300"
					onmouseenter={showText} onmouseleave={hideText}
				>
					<h3 class="mb-2 text-xl font-medium">Precise Search</h3>
					<p class="text-gray-400">Quickly find the data you need with precision.</p>
					<p class="mt-4 hidden text-gray-300">
						Our powerful search instantly scans and retrieves relevant financial data from multiple
						sources, helping you cut through the noise. Whether you're searching for company
						fundamentals, stock performance, or breaking financial news, our system delivers precise
						and up-to-date results. With advanced filtering and sorting options, you can quickly
						refine your search and access the critical information you need—without endless
						scrolling or guesswork.
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
	.feature-card{
		transition: transform 0.2s ease-in-out;
	}

	.feature-card:hover {
		transform: scale(1.05);
		background-color: #4c3c88;
	}

	/* Add margin to the left of the first card and right of the last card */
	.feature-card:first-child {
		margin-left: 1rem;
	}

	.feature-card:last-child {
		margin-right: 1rem;
	}
</style>
