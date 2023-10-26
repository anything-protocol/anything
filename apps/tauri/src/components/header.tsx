import { VscCode, VscDebug, VscGear,VscRepoForked } from "react-icons/vsc";
import { useParams } from "react-router-dom";

import { useFlowNavigationContext } from "../context/FlowNavigationProvider";

export default function Header() {
  const {
    nodePanel,
    setNodePanel,
    tomlPanel,
    setTomlPanel,
    debugPanel,
    setDebugPanel,
    setSettingsPanel,
    settingsPanel,
  } = useFlowNavigationContext();

  const { flow_name } = useParams();

  return (
    <div className="w-full z-10 bg-primary pl-2 text-white overflow-hidden">
      <div className="flex flex-row">
        <div className="">flows/{flow_name}</div>
        <div className="flex-grow" />
        <button onClick={() => setNodePanel(!nodePanel)}>
          <VscRepoForked className="mr-2 h-5 w-5" />
        </button>
        <button onClick={() => setDebugPanel(!debugPanel)}>
          <VscDebug className="mr-2 h-4 w-5" />
        </button>
        <button onClick={() => setTomlPanel(!tomlPanel)}>
          <VscCode className="mr-2 h-5 w-5" />
        </button>
        <button onClick={() => setSettingsPanel(!settingsPanel)}>
          <VscGear className="mr-2 h-5 w-5" />
        </button>
      </div>
    </div>
  );
}
