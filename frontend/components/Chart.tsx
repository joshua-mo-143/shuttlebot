"use client"
import Link from 'next/link'
import React from 'react'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { Line } from 'react-chartjs-2';

type Props = {
	data: IssuesOpenedLastWeek[] | undefined
}

interface IssuesOpenedLastWeek {
  day: string,
  totalIssuesPerDay: number,
}


export default function Chart({data}: Props) {
	
ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

	const options = {
  responsive: true,
  plugins: {
    legend: {
			display: false,
    },
    title: {
      display: false,
    },
  },
};

	let labels = data?.map((item) => item.day);



	const chartData = {
		labels,
		datasets: [{
			label: 'Dataset 1',
			data: data?.map((item) => item.totalIssuesPerDay),
			borderColor: 'rgb(255, 99, 132)',
      backgroundColor: 'rgba(255, 99, 132, 0.5)',
		}]
	}
	
	return (
		<Line options={options} data={chartData}/>
	)
}

