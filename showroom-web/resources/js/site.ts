import "../../bindings/boutique/style/index.css";
import "../../bindings/boutique/style/components.css";
import "../css/base.css";
import "../css/app.css";
import "../css/prose.css";


import { defineSetups } from "../../bindings/boutique/setups";
import { registry } from "../../bindings/registry";

import.meta.glob("../../src/components/**/index.css", { eager: true });

defineSetups(registry);
