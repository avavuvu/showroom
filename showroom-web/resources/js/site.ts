import "@bq/style/reset.css";
import "@bq/style/tokens.css";
import "@bq/style/roles.css";
import "../css/app.css";
import "../css/prose.css";

import.meta.glob("@bq/components/src/**/*.css", { eager: true });
import.meta.glob(["@bq/components/src/**/*.ts", "!@bq/components/src/**/_*.ts"], { eager: true });
