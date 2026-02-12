import type {Metadata} from "next";
import {TRPCProvider} from "@/trpc/provider";
import "./globals.css";

export const metadata: Metadata = {
    title: "Next.js + tRPC App",
    description: "A Next.js 16 app with tRPC, Tailwind CSS, and Drizzle ORM",
};

export default function RootLayout({
                                       children,
                                   }: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en">
        <body>
        <TRPCProvider>{children}</TRPCProvider>
        </body>
        </html>
    );
}
