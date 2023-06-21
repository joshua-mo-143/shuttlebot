// "use client"
// import Link from 'next/link'
// import React from 'react'
// import {useRouter} from 'next/navigation'

// export default function Search() {
// 	// let router = useRouter()
	
// 	const [data, setData] = React.useState<string>("");

// 	// const handleSubmit = (e: React.SyntheticEvent) => {
// 	// 	e.preventDefault()

// 	// 	let params = new URLSearchParams(router.pathname);
		
// 	// 	params.append("hello", "world");

// 	// }


// 	return (
// 		<>
// 		<form className="w-full flex flex-row justify-center gap-10 py-10">
// 			<label htmlFor="answered" className="px-5 py-2 flex flex-row justify-center gap-4 items-center">
// 					<span>Answered</span>
// 				<select name="answered" className="px-5 py-2 rounded-md bg-slate-600">
// 				<option value=""></option>
// 				<option value={true}>True</option>
// 				<option value={false}>False</option>
// 				</select>
// 				</label>
// 			<label htmlFor="resolved" className="px-5 py-2 flex flex-row justify-center gap-4 items-center">
// 					<span>Resolved</span>
// 				<select name="resolved" className="px-5 py-2 rounded-md bg-slate-600">
// 				<option value=""></option>
// 				<option value={true}>True</option>
// 				<option value={false}>False</option>
// 				</select>
// 				</label>
// 			<label htmlFor="elevated" className="px-5 py-2 flex flex-row justify-center gap-4 items-center">
// 					<span>Elevated</span>
// 				<select name="elevated" className="px-5 py-2 rounded-md bg-slate-600">
// 				<option value=""></option>
// 				<option value={true}>True</option>
// 				<option value={false}>False</option>
// 				</select>
// 				</label>
// 				<button type="submit">Search!</button>
// 		</form>
// 	</>
// 	)
// }