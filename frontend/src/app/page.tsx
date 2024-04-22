import type { Metadata } from "next";
import { Inter } from "next/font/google";
import VideoBox from "./video"
import  { VideoBoxProp } from "./video";
import "./globals.css";
import Image from "next/image"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { ModeToggle } from "@/components/mode-toggle"

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
    title: "Void Pixel",
    description: "Generated by create next app",
};

export default function Home() {
    const videos: VideoBoxProp[] = [];
    for (let i: number = 0; i < 30; i++){
        videos.push({
            title: "Example Video Title",
            creator: "Example Creator Name",
            src: "/static/images/1.jpg",
        });
    }
    return (
        <body className={inter.className}>
            <div className="flex items-center overflow-hidden justify-between m-4 max-w-screen">
                <div className="flex items-center">
                    <Avatar>
                        <AvatarImage src="https://github.com/shadcn.png" />
                        <AvatarFallback>CN</AvatarFallback>
                    </Avatar>
                    <span className="ml-3">User</span> 
                </div>
                <div className="flex flex-grow justify-center">
                    <Input type="text" className="px-3 py-2 w-1/2" placeholder="Search..." />
                    <Button className="ml-2 px-3 py-2">Search</Button>
                </div>
                <div className="text-right">Void Pixel</div>
                <div className="flex ml-30">
                    <ModeToggle />
                </div>
            </div>

            <div id="video_container"className="flex flex-wrap m-3 mt-10 justify-center">
                {videos.map( video => (
                    <VideoBox title={video.title} creator={video.creator} src={video.src}/>
                ))}
            </div>

        </body>
    );
}
