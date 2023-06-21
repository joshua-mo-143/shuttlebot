"use client"
import React from 'react'
import Link from 'next/link'

interface Record {
	id: number,
	originalPoster: string,
	severity: number,
	firstResponder?: string,
	resolvedBy?: string,
	discordThreadLink: string,
	githubLink?: string,
	creationDate: string,
}

export default function Issues() {

	const [data, setData] = React.useState<Record[]>([]);

	React.useEffect(() => {
		const meme = async () => {
			
	let fetch_url = `//localhost:8000/api/issues`

		try {
			let res = await fetch(fetch_url, {
					mode: "cors"
				});
			let json = await res.json();
			setData(json)
		} catch (e: any) {
				console.log(e.message)
			}
		}
		meme()
	}, [])

	return (
	
		<div>
			<h1 className="text-2xl text-center">Issues</h1>
			{data ?
				<table className="text-center">
				<thead>
					<tr>
						<th className="px-5 py-2">Original Poster</th>
						<th className="px-5 py-2">Severity</th>
						<th className="px-5 py-2">First Responder</th>
						<th className="px-5 py-2">Resolved By</th>
						<th className="px-5 py-2">Discord Thread Link</th>
						<th className="px-5 py-2">Github Link</th>
						<th className="px-5 py-2">Created At</th>
					</tr>
		</thead>
				<tbody>
				{data.map((item) => (
		<tr key={item.id}>
			<td className="px-5">
							{item.originalPoster}
							</td>
			<td className="px-5">
							{item.severity}
							</td>
			<td className="px-5">
							{item.firstResponder ? item.firstResponder : "No response yet"}
							</td>
			<td className="px-5">
							{item.resolvedBy ? item.resolvedBy : "Not resolved yet"}
							</td>
			<td className="px-5">
							<Link href={item.discordThreadLink} target="_blank">Click</Link>
							</td>
			<td className="px-5">
							{item.githubLink ? 
							<Link href={item.githubLink} target="_blank">Click</Link>
							: "Not elevated"}
							</td>
			<td className="px-5">
							{item.creationDate}
							</td>
			</tr>
	))}
		</tbody>
				</table> : <p> Data fetching failed :( </p>}
		</div>
	)
}
