import { Link, useNavigate } from "react-router-dom";

import BaseCard from "../components/baseCard";
import { useFlowsContext } from "../context/FlowsProvider";
import PageLayout from "../pageLayout";
import { PageHeader } from "../components/wholePageHeader";

export default function Home() {
  const { flows, createNewFlow } = useFlowsContext();
  const navigate = useNavigate();

  return (
    <PageLayout>
      {/* FLows */}
      <div className="flex flex-row w-full h-full">
        <div className="flex flex-col w-1/3">
          <PageHeader
            title="Flows"
            callback={createNewFlow}
            buttonLabel="New Flow"
          />
          <ul>
            {flows.map((flow) => {
              return (
                <BaseCard
                  as={Link}
                  key={flow.flowName}
                  to={`flows/${flow.flowName}`}
                  className="w-96"
                >
                  <h2 className="card-title">{flow.flowName}</h2>
                </BaseCard>
              );
            })}
          </ul>
        </div>

        {/* Tables */}
        <div className="flex flex-col w-1/3 pl-10">
          <PageHeader
            title="Templates"
            callback={() => navigate("/templates")}
            buttonLabel="Explore"
          />
          <ul></ul>
        </div>
      </div>
    </PageLayout>
  );
}
