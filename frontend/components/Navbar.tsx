"use client"
import Link from 'next/link'

export default function Navbar() {

	return (
		<nav className="h-10 w-full bg-slate-700/50">
			<ul className="flex flex-row justify-center w-full items-center gap-10 p-2">
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