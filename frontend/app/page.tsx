"use client"
import Image from 'next/image'
import Navbar from '@/components/Navbar'

export default function Home() {
  return (
        <div>
          <p> Monthly Stats </p>
          <div>
            <p>Tickets resolved this month:</p>
            </div> 
          <div>
            <p>Total tickets opened this month:</p>
        </div>
          <div>
            <p>Help threads elevated to Github Issues:</p>
        </div>
  </div>
  )
}
