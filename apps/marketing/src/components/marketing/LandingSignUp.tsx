"use client";

// import { signIn } from "next-auth/react";

import { Button } from "@repo/ui/components/ui/button";

export function SignUpButton({ className }: { className: string }) {
  async function handleGithub() {
    // signIn("github", { callbackUrl: "/generate" });
  }

  return (
    <div className="flex flex-col items-center gap-2 md:flex-row md:gap-4">
      {/* <Button
        type="button"
        variant="primary"
        className={className}
        onClick={() => {
          handleGithub();
        }}
      >
        Sign up with Github
      </Button> */}
    </div>
  );
}
