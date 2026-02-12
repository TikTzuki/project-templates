"use client";

import {trpc} from "@/trpc/client";

export default function Home() {
    const hello = trpc.example.hello.useQuery({name: "World"});

    return (
        <main className="flex min-h-screen flex-col items-center justify-center gap-4 p-8">
            <h1 className="text-4xl font-bold">Hello World</h1>
            <p className="text-lg text-gray-600">
                {hello.isLoading
                    ? "Loading..."
                    : hello.data
                        ? hello.data.greeting
                        : "Error loading greeting"}
            </p>
        </main>
    );
}
