"use client";

import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";

import { Button } from "@repo/ui/components/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@repo/ui/components/ui/form";
import { Input } from "@repo/ui/components/ui/input";

import { useState } from "react";
import { Edit2, Trash2 } from "lucide-react";
import { TableCell, TableRow } from "@repo/ui/components/ui/table";

//Use same schema for both
const formSchema = z.object({
  secret_name: z
    .string()
    .regex(/^[A-Z0-9]+(?:_{0,2}[A-Z0-9]+)*$/, {
      message: "Must be all caps and SNAKE_CASE example: OPENAI_API_KEY",
    })
    .max(100),
  secret_value: z.string().min(2).max(400),
  secret_description: z.string().max(100).optional(),
});

export function CreateNewSecret({ cancel, saveSecret }: any): JSX.Element {
  // 1. Define your form.
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      secret_name: "",
      secret_value: "",
      secret_description: "",
    },
  });

  // 2. Define a submit handler.
  async function onSubmit(values: z.infer<typeof formSchema>) {
    // console.log(values)
    await saveSecret(
      values.secret_name,
      values.secret_value,
      values.secret_description,
    );
    form.reset();
    cancel();
  }

  return (
    <div className="flex flex-row max-w-5xl mx-auto">
      <div className="flex-1 flex flex-row max-w-5xl mx-auto border p-7 rounded-md m-7">
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
            <div className="flex flex-row">
              <FormField
                control={form.control}
                name="secret_name"
                render={({ field }) => (
                  <FormItem className="mr-10">
                    <FormLabel>Name</FormLabel>
                    <FormControl>
                      <Input placeholder="OPENAI_API_KEY" {...field} />
                    </FormControl>
                    <FormDescription>
                      This is the public name you will use to refer to this
                      secret.
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="secret_value"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Value</FormLabel>
                    <FormControl>
                      <Input placeholder="pk_0498234i-0i" {...field} />
                    </FormControl>
                    <FormDescription>
                      Stored enrypted in the database for use in your
                      automations.
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <FormField
              control={form.control}
              name="secret_description"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <Input
                      placeholder="Our api key for interacting with ..."
                      {...field}
                    />
                  </FormControl>
                  <FormDescription>
                    One line note to help you and AI recall how this is used.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button variant={"secondary"} className="mr-2" onClick={cancel}>
              Cancel
            </Button>
            <Button type="submit">Save</Button>
          </form>
        </Form>
      </div>
    </div>
  );
}

export function EditSecret({
  secret,
  deleteSecret,
  updateSecret,
}: any): JSX.Element {
  const [editing, setEditing] = useState(false);

  // 1. Define your form.
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      secret_name: secret.secret_name,
      secret_value: secret.secret_value,
      secret_description: secret.secret_description,
    },
  });

  // 2. Define a submit handler.
  async function onSubmit(values: z.infer<typeof formSchema>) {
    // Do something with the form values.
    // ✅ This will be type-safe and validated.
    // console.log(values)
    await updateSecret(
      secret.secret_id,
      values.secret_name,
      values.secret_value,
      values.secret_description,
    );
    setEditing(false);
  }

  return (
    <TableRow className="flex flex-row max-w-5xl mx-auto">
      {/* {secret && !editing ? (
        <> */}
      <TableCell>{secret.secret_name}</TableCell>
      <TableCell>{secret.secret_description}</TableCell>
      <TableCell>{secret.secret_value}</TableCell>

      <TableCell className="text-right">
        <Button
          variant="outline"
          size="sm"
          className="ml-2"
          onClick={() => setEditing(!editing)}
        >
          <Edit2 className="size-5" />
        </Button>
        <Button
          variant="outline"
          size="sm"
          className="ml-2"
          onClick={() => deleteSecret(secret)}
        >
          <Trash2 className="size-5" />
        </Button>
      </TableCell>
    </TableRow>
  );
}

// </>
//   ) : (
//     <div className="flex-1 flex flex-row max-w-5xl mx-auto border p-7 rounded-md m-7">
//       <Form {...form}>
//         <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
//           <div className="flex flex-row">
//             <FormField
//               disabled
//               control={form.control}
//               name="secret_name"
//               render={({ field }) => (
//                 <FormItem className="mr-10">
//                   <FormLabel>Name</FormLabel>
//                   <FormControl>
//                     <Input placeholder="OPENAI_API_KEY" {...field} />
//                   </FormControl>
//                   <FormDescription>
//                     This is the public name you will use to refer to this
//                     secret. Cannot be updated.
//                   </FormDescription>
//                   <FormMessage />
//                 </FormItem>
//               )}
//             />
//             <FormField
//               control={form.control}
//               name="secret_value"
//               render={({ field }) => (
//                 <FormItem>
//                   <FormLabel>Value</FormLabel>
//                   <FormControl>
//                     <Input placeholder="pk_0498234i-0i" {...field} />
//                   </FormControl>
//                   <FormDescription>
//                     Stored enrypted in the database for use in your
//                     automations.
//                   </FormDescription>
//                   <FormMessage />
//                 </FormItem>
//               )}
//             />
//           </div>

//           <FormField
//             control={form.control}
//             name="secret_description"
//             render={({ field }) => (
//               <FormItem>
//                 <FormLabel>Description</FormLabel>
//                 <FormControl>
//                   <Input
//                     placeholder="Our api key for interacting with ..."
//                     {...field}
//                   />
//                 </FormControl>
//                 <FormDescription>
//                   One line note to help you recall how this is used.
//                 </FormDescription>
//                 <FormMessage />
//               </FormItem>
//             )}
//           />
//           <Button
//             variant={"secondary"}
//             className="mr-2"
//             onClick={() => setEditing(false)}
//           >
//             Cancel
//           </Button>
//           <Button type="submit">Save</Button>
//         </form>
//       </Form>
//     </div>
//   )}
