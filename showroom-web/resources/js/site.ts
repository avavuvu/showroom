import "../css/app.css";
import "../css/prose.css";

import.meta.glob("@bq/**/*.css", { eager: true });
import.meta.glob(["@bq/**/*.ts", "!@bq/**/_*.ts"], { eager: true });
