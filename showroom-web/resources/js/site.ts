import "@bq/style/reset.css";
import "@bq/style/tokens.css";
import "@bq/style/roles.css";
import "../css/base.css";
import "../css/app.css";
import "../css/prose.css";

import "@bq/components/src/autosave/autosave";
import "@bq/components/src/behaviours/behaviours";
import { defineSetups } from "@bq/components/src/setups";
import { registry } from "../../bindings/registry";

import.meta.glob("../../src/components/*/index.css", { eager: true });

defineSetups(registry);
