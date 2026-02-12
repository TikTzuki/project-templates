import {router} from "@/server/trpc";
import {exampleRouter} from "@/server/routers/example";

export const appRouter = router({
    example: exampleRouter,
});

export type AppRouter = typeof appRouter;
