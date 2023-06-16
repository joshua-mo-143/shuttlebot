"use client"
import Link from 'next/link'

export default function Navbar() {

	return (
		<nav className="min-h-screen col-span-1 bg-sky-200">
			<ul className="flex flex-col justify-center w-full items-center p-2">
			<li>
					<Link href="/">Monthly Stats</Link>
			</li>
			<li>
					<Link href="/issues">Issues</Link>
			</li>
			</ul>
		</nav>
	)
}