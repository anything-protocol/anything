import { notFound } from "next/navigation";

import { TemplateGrid } from "@/components/templateGrid";
import { fetchTemplates } from "@/lib/fetchSupabase";

export default async function TemplatePage() {
  const templates = await fetchTemplates();

  if (!templates) {
    notFound();
  }

  return (
    <>
      {/* Hero Copy */}
      <div className="mt-16 flex flex-col items-center gap-4">
        <h1 className="md:h1 h2 w-full px-4 text-center md:w-[805px] md:px-0">
          Anything Templates
        </h1>
        <p className="body-xl w-full px-4 text-center text-slate-11 md:w-[572px] md:px-0">
          Automate anything with ready to use templates
        </p>
      </div>

      {/* Grid */}
      <div className="my-16 flex flex-col items-center">
        <TemplateGrid templates={templates} />
      </div>
    </>
  );
}