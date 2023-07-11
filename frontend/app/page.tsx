"use client"
import Image from 'next/image'
import Navbar from '@/components/Navbar'
import React from 'react'
import Link from 'next/link'
import Chart from '@/components/Chart'

interface DashboardData {
  lastFourWeeksStats: LastFourWeeksStats[],
  issuesAwaitingResponse: IssuesAwaitingResponse,
  issuesOpenedLastWeek: IssuesOpenedLastWeek[],
}

interface LastFourWeeksStats {
  dateRange: string,
  totalIssues: number,
  totalElevatedIssues: number,
  totalResolvedIssues: number,
  totalOneTouchThreads: number,
  extendedThreads: number,
  averageResponseTime: string,
  bestSolver?: string,
  bestFirstResponder?: string
}

interface IssuesOpenedLastWeek {
  day: string,
  totalIssuesPerDay: number,
}

interface IssuesAwaitingResponse {
  unansweredThreads: number,
  unresolvedIssues: number,
  unresolvedGithubIssues: number
}

export default function Home() {

  const [data, setData] = React.useState<DashboardData>();

	React.useEffect(() => {
		const meme = async () => {
			
	let fetch_url = `//${window.location.host}/api/dashboard`

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
      <div className="grid grid-cols-1 grid-rows-auto gap-2">
      <div className="p-4 px-10 border border-2 rounded-md w-[100%] flex flex-col gap-4 col-span-1">
      <h2 className="text-left text-2xl">Currently Outstanding</h2>
        <div className="grid grid-cols-3 grid-rows-1 gap-4 text-sm">
          <div className="col-span-1">
              <p className="text-5xl">{data?.issuesAwaitingResponse?.unansweredThreads}</p>
            <p>Unanswered Issues</p>
            </div>
          <div className="col-span-1">
            <p className="text-5xl">{data?.issuesAwaitingResponse?.unresolvedIssues}</p>
            <p>Unresolved Issues</p>
            </div>
          <div className="col-span-1">
            <p className="text-5xl">{data?.issuesAwaitingResponse?.unresolvedGithubIssues}</p>
            <p>Unresolved GitHub Issues</p>
            </div>
          </div>
      </div>
        
      <div className="p-4 px-10 border border-2 rounded-md w-[100%] flex flex-col gap-4 col-span-1">
      <h2 className="text-left text-2xl">Stats - Last 4 Weeks</h2>
          <table className="text-center">
            <thead>
              <tr>
                <th className="px-2">Date Range</th>
                <th className="px-2">Total Issues</th>
                <th className="px-2">Total Elevated</th>
                <th className="px-2">Total Resolved</th>
                <th className="px-2">Average Response Time</th>
                <th className="px-2">Best Solver</th>
                <th className="px-2">Best First Responder</th>
              </tr>
            </thead>
            <tbody>
              {data?.lastFourWeeksStats?.map((item) => (
                <tr key={item.dateRange}>
                  <td className="px-2">{item.dateRange}</td>
                  <td className="px-2">{item.totalIssues}</td>
                  <td className="px-2">{item.totalElevatedIssues}</td>
                  <td className="px-2">{item.totalResolvedIssues}</td>
                  <td className="px-2">{item.averageResponseTime}</td>
                  <td className="px-2">{item.bestSolver ? item.bestSolver : "No issues solved yet"}</td>
                  <td className="px-2">{item.bestFirstResponder ? item.bestFirstResponder : "No help threads responded to yet :("}</td>
                  </tr>
              ))}
            </tbody>
        </table>
      </div>

      <div className="p-4 px-10 border border-2 rounded-md w-[100%] flex flex-col gap-4 col-span-1">
      <h2 className="text-center text-2xl">Issues Opened over last 7 Days</h2>
          <Chart data={data?.issuesOpenedLastWeek}/>
      </div>
        
      <div className="p-4 px-10 border border-2 rounded-md w-[100%] flex flex-col gap-4 col-span-1">
      <h2 className="text-left text-2xl">Open Pull Requests</h2>
        <div className="grid grid-cols-4 grid-rows-1 gap-4 text-sm">
          <Link className="col-span-1" href="https://github.com/shuttle-hq/shuttle/pulls" target="_blank">
            <p className="text-5xl">0</p>
            <p>Main repo</p>
            </Link>
          <Link className="col-span-1" href="https://github.com/shuttle-hq/shuttle-docs/pulls" target="_blank">
            <p className="text-5xl">0</p>
            <p>Docs repo</p>
            </Link>
          <Link className="col-span-1" href="https://github.com/shuttle-hq/shuttle-examples/pulls" target="_blank">
            <p className="text-5xl">0</p>
            <p>Examples repo</p>
            </Link>
          <Link className="col-span-1" href="https://github.com/shuttle-hq/deploy-action/pulls" target="_blank">
            <p className="text-5xl">0</p>
            <p>Deploy action</p>
            </Link>
          </div>
      </div>
      </div>
      // <Link href="https://github.com/login/oauth/authorize?client_id=a943aff4893a533b6cb9">Log In with Github</Link>
  </div>
  )
}

