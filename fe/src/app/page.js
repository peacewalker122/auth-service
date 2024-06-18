"use client";
import { useState } from "react";

export default function Home() {
	const [username, setUsername] = useState("");
	const [password, setPassword] = useState("");

	const handleSubmit = (event) => {
		event.preventDefault();
		// Handle the login logic here
		console.log("Username:", username);
		console.log("Password:", password);
	};

	return (
		<main className="flex flex-col justify-center items-center h-screen bg-indigo-900">
			<div className="w-96 p-6 bg-white shadow-lg rounded-md">
				<h1 className="text-3xl block text-center font-mono text-black font-semibold">
					Login
				</h1>

				<form onSubmit={handleSubmit}>
					<div className="mt-3">
						<label htmlFor="username" className="block text-base mb-2">
							Username
						</label>
						<input
							type="text"
							id="username"
							className="text-black border w-full text-base px-2 py-2 focus:border-gray focus:outline-none focus:ring-0 rounded-md"
							placeholder="Enter Username"
							value={username}
							onChange={(e) => setUsername(e.target.value)}
						/>
						<label htmlFor="password" className="block text-base mb-2">
							Password
						</label>
						<input
							type="password"
							id="password"
							className="text-black border w-full text-base px-2 py-2 focus:border-gray focus:outline-none focus:ring-0 rounded-md"
							placeholder="Enter Password"
							value={password}
							onChange={(e) => setPassword(e.target.value)}
						/>
					</div>

					<div className="flex flex-row my-4 py-2 justify-between">
						<span>
							<input type="checkbox" />
							<label className="text-indigo-600 ml-2 text-2md">
								Remember Me?
							</label>
						</span>
						<span>
							<a href="/" className="text-indigo-600 text-2md">
								Need Help?
							</a>
						</span>
					</div>

					<button
						type="submit"
						className="border-2 w-full border-indigo-700 bg-indigo-700 text-white py-1 px-2 rounded-md"
						onClick={(e) => handleSubmit(e)}
					>
						Login
					</button>
				</form>
			</div>
		</main>
	);
}
