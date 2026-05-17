import { useQuery } from "@tanstack/react-query";
import { useMemo, useState } from "react";

import { invoke } from "@/adapters";
import { SwipablePage, SwipablePageView } from "@/components/page";
import { Badge } from "@wealthfolio/ui/components/ui/badge";
import { Button } from "@wealthfolio/ui/components/ui/button";
import { EmptyPlaceholder } from "@wealthfolio/ui/components/ui/empty-placeholder";
import { Icons } from "@wealthfolio/ui/components/ui/icons";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@wealthfolio/ui/components/ui/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@wealthfolio/ui/components/ui/table";

type IpoMarket = "hk" | "cn" | "cb" | "reits" | "us";

interface HkIpoRecord {
  id: string;
  code: string;
  name: string;
  board: string;
  subscriptionStart: string;
  subscriptionEnd: string;
  listingDate: string;
  issuePrice: string;
  issueSize: string;
  lotSize: string;
  winRate: string;
  firstDayChange: string;
  underwriter: string;
  status: "upcoming" | "open" | "closed" | "listed";
}

interface AShareIpoRecord {
  id: string;
  code: string;
  name: string;
  board: string;
  subscriptionDate: string;
  issuePrice: string;
  subscriptionCode: string;
  pe: string;
  maxSubscriptions: string;
  listingDate: string;
  onlineRate: string;
  topMarketValue: string;
  issueSize: string;
  industryPe: string;
  onlineIssue: string;
  sponsor: string;
  status: "upcoming" | "open" | "closed" | "listed";
  prospectusUrl?: string;
}

interface CbRecord {
  id: string;
  code: string;
  name: string;
  progress: string;
  announceDate: string;
  issueSize: string;
  stockPrice: string;
  conversionPrice: string;
  conversionPremium: string;
  term: string;
  minMarketValue: string;
}

interface ReitsRecord {
  id: string;
  code: string;
  name: string;
  assetType: string;
  subscriptionDate: string;
  issuePrice: string;
  subscriptionCode: string;
  maxSubscriptions: string;
  listingDate: string;
  underlyingAsset: string;
  sponsor: string;
  status: "upcoming" | "open" | "closed" | "listed";
}

interface IpoRecord {
  id: string;
  name: string;
  symbol: string;
  ipoDate: string;
  issuePrice: string;
  subscriptionStart: string;
  subscriptionEnd: string;
  status: "upcoming" | "open" | "closed" | "listed";
}

const STATUS_VARIANTS: Record<
  "upcoming" | "open" | "closed" | "listed",
  "default" | "secondary" | "destructive" | "outline"
> = {
  upcoming: "outline",
  open: "secondary",
  closed: "default",
  listed: "default",
};

const STATUS_LABELS: Record<"upcoming" | "open" | "closed" | "listed", string> = {
  upcoming: "即将申购",
  open: "申购中",
  closed: "已结束",
  listed: "已上市",
};

const HK_MARKET_COLUMNS = [
  "股票代码",
  "股票名称",
  "板块",
  "招股日期",
  "截止日期",
  "公布日期",
  "暗盘日期",
  "上市日期",
  "发行价(HKD)",
  "募资规模(亿HKD)",
  "一手入场费",
  "中签率",
  "首日涨幅",
  "保荐人",
  "状态",
];

const A_SHARE_COLUMNS = [
  "证券代码",
  "证券简称",
  "板块",
  "申购日期",
  "发行价(元)",
  "申购代码",
  "发行市盈率",
  "申购上限(万股)",
  "上市日期",
  "中签率",
  "顶格市值(万)",
  "发行规模(万股)",
  "行业市盈率",
  "网上发行(万股)",
  "保荐人",
  "状态",
];

const CB_COLUMNS = [
  "股票代码",
  "股票名称",
  "审批进度",
  "公告日期",
  "发行规模(亿)",
  "正股价(元)",
  "转股价(元)",
  "转股溢价率",
  "发行期限(年)",
  "持仓市值(万)",
];

const REITS_COLUMNS = [
  "基金代码",
  "基金简称",
  "底层资产类型",
  "认购日期",
  "发售价格(元)",
  "认购代码",
  "认购上限(万份)",
  "上市日期",
  "底层资产",
  "管理人",
  "状态",
];

const MARKET_COLUMNS: Record<IpoMarket, string[]> = {
  hk: ["股票名称", "股票代码", "招股日期", "截止日期", "发行价(HKD)", "上市日期", "状态"],
  cn: ["股票名称", "股票代码", "申购日期", "截止日期", "发行价(CNY)", "上市日期", "状态"],
  cb: [],
  reits: [],
  us: ["股票名称", "股票代码", "路演开始", "定价日期", "发行价(USD)", "上市日期", "状态"],
};

const CB_PROGRESS_VARIANTS: Record<string, "default" | "secondary" | "destructive" | "outline"> = {
  同意注册: "default",
  上市委通过: "secondary",
  交易所受理: "secondary",
  股东大会通过: "outline",
  董事会预案: "outline",
};

const DEMO_DATA: Record<IpoMarket, IpoRecord[]> = {
  hk: [],
  cn: [],
  cb: [],
  reits: [],
  us: [],
};

const getPreviousNthTradingDay = (date: string, n: number) => {
  if (!date || date === "-") {
    return "-";
  }
  const m = date.match(/(\d{1,2})-(\d{1,2})/);
  if (!m) {
    return "-";
  }
  const month = Number(m[1]);
  const day = Number(m[2]);
  const year = new Date().getFullYear();
  const current = new Date(year, month - 1, day);
  let moved = 0;
  while (moved < n) {
    current.setDate(current.getDate() - 1);
    const weekday = current.getDay();
    if (weekday !== 0 && weekday !== 6) {
      moved += 1;
    }
  }
  const mm = String(current.getMonth() + 1).padStart(2, "0");
  const dd = String(current.getDate()).padStart(2, "0");
  return `${mm}-${dd}`;
};

const formatDateWithWeekday = (value: string) => {
  if (!value || value === "-") {
    return "-";
  }
  const cleaned = value.replace(/<[^>]*>/g, "").trim();
  const match = cleaned.match(
    /(\d{4}-\d{1,2}-\d{1,2}|\d{1,2}-\d{1,2})\s*\((周[一二三四五六日天])\)/,
  );
  if (match) {
    return `${match[1]}(${match[2]})`;
  }
  return cleaned;
};

const IpoTable = ({ market }: { market: IpoMarket }) => {
  const { data: hkIpos = [], isLoading: hkLoading } = useQuery<HkIpoRecord[]>({
    queryKey: ["hk_ipos"],
    queryFn: () => invoke("get_hk_ipos"),
    enabled: market === "hk",
  });
  const [hkPageIndex, setHkPageIndex] = useState(0);
  const [hkPageSize, setHkPageSize] = useState(10);

  const { data: cnIpos = [], isLoading: cnLoading } = useQuery<AShareIpoRecord[]>({
    queryKey: ["cn_ipos"],
    queryFn: () => invoke("get_cn_ipos"),
    enabled: market === "cn",
  });
  const [cnPageIndex, setCnPageIndex] = useState(0);
  const [cnPageSize, setCnPageSize] = useState(10);

  const { data: reitsIpos = [], isLoading: reitsLoading } = useQuery<ReitsRecord[]>({
    queryKey: ["reits_ipos"],
    queryFn: () => invoke("get_reits_ipos"),
    enabled: market === "reits",
  });
  const { data: cbIpos = [], isLoading: cbLoading } = useQuery<CbRecord[]>({
    queryKey: ["cb_ipos"],
    queryFn: () => invoke("get_cb_ipos"),
    enabled: market === "cb",
  });
  const [reitsPageIndex, setReitsPageIndex] = useState(0);
  const [reitsPageSize, setReitsPageSize] = useState(10);
  const [cbPageIndex, setCbPageIndex] = useState(0);
  const [cbPageSize, setCbPageSize] = useState(10);

  if (market === "cb") {
    if (cbLoading) {
      return (
        <div className="flex items-center justify-center py-16">
          <Icons.Spinner className="text-muted-foreground h-6 w-6 animate-spin" />
        </div>
      );
    }

    const cbTotalPages = Math.max(1, Math.ceil(cbIpos.length / cbPageSize));
    const cbPagedIpos = cbIpos.slice(cbPageIndex * cbPageSize, (cbPageIndex + 1) * cbPageSize);

    return (
      <div className="space-y-2">
        <div className="overflow-x-auto rounded-md border">
          <Table className="min-w-[1100px]">
            <TableHeader>
              <TableRow>
                {CB_COLUMNS.map((column) => (
                  <TableHead key={column}>{column}</TableHead>
                ))}
              </TableRow>
            </TableHeader>
            <TableBody>
              {cbPagedIpos.map((record) => (
                <TableRow key={record.id}>
                  <TableCell className="font-medium">{record.code}</TableCell>
                  <TableCell>{record.name}</TableCell>
                  <TableCell>
                    <Badge variant={CB_PROGRESS_VARIANTS[record.progress] ?? "outline"}>
                      {record.progress}
                    </Badge>
                  </TableCell>
                  <TableCell>{record.announceDate}</TableCell>
                  <TableCell>{record.issueSize}</TableCell>
                  <TableCell>{record.stockPrice}</TableCell>
                  <TableCell>{record.conversionPrice}</TableCell>
                  <TableCell>{record.conversionPremium}</TableCell>
                  <TableCell>{record.term}</TableCell>
                  <TableCell>{record.minMarketValue}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
        <div className="flex items-center justify-between px-1">
          <div className="text-muted-foreground text-sm">
            共 {cbIpos.length} 条，第 {cbPageIndex + 1} / {cbTotalPages} 页
          </div>
          <div className="flex items-center gap-2">
            <Select
              value={String(cbPageSize)}
              onValueChange={(v) => {
                setCbPageSize(Number(v));
                setCbPageIndex(0);
              }}
            >
              <SelectTrigger className="h-8 w-[80px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {[10, 20, 50].map((s) => (
                  <SelectItem key={s} value={String(s)}>
                    {s} 条
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setCbPageIndex((i) => Math.max(0, i - 1))}
              disabled={cbPageIndex === 0}
            >
              上一页
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setCbPageIndex((i) => Math.min(cbTotalPages - 1, i + 1))}
              disabled={cbPageIndex >= cbTotalPages - 1}
            >
              下一页
            </Button>
          </div>
        </div>
      </div>
    );
  }

  if (market === "cn") {
    if (cnLoading) {
      return (
        <div className="flex items-center justify-center py-16">
          <Icons.Spinner className="text-muted-foreground h-6 w-6 animate-spin" />
        </div>
      );
    }
    const cnTotalPages = Math.max(1, Math.ceil(cnIpos.length / cnPageSize));
    const cnPagedIpos = cnIpos.slice(cnPageIndex * cnPageSize, (cnPageIndex + 1) * cnPageSize);
    return (
      <div className="space-y-2">
        <div className="overflow-x-auto rounded-md border">
          <Table className="min-w-[1400px]">
            <TableHeader>
              <TableRow>
                {A_SHARE_COLUMNS.map((column) => (
                  <TableHead key={column}>{column}</TableHead>
                ))}
              </TableRow>
            </TableHeader>
            <TableBody>
              {cnPagedIpos.map((record) => (
                <TableRow key={record.id}>
                  <TableCell className="font-medium">{record.code}</TableCell>
                  <TableCell>{record.name}</TableCell>
                  <TableCell>{record.board}</TableCell>
                  <TableCell>{record.subscriptionDate}</TableCell>
                  <TableCell>{record.issuePrice}</TableCell>
                  <TableCell>{record.subscriptionCode}</TableCell>
                  <TableCell>{record.pe}</TableCell>
                  <TableCell>{record.maxSubscriptions}</TableCell>
                  <TableCell>{formatDateWithWeekday(record.listingDate)}</TableCell>
                  <TableCell>{record.onlineRate}</TableCell>
                  <TableCell>{record.topMarketValue}</TableCell>
                  <TableCell>{record.issueSize}</TableCell>
                  <TableCell>{record.industryPe}</TableCell>
                  <TableCell>{record.onlineIssue}</TableCell>
                  <TableCell>{record.sponsor}</TableCell>
                  <TableCell>
                    <Badge variant={STATUS_VARIANTS[record.status]}>
                      {STATUS_LABELS[record.status]}
                    </Badge>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
        <div className="flex items-center justify-between px-1">
          <div className="text-muted-foreground text-sm">
            共 {cnIpos.length} 条，第 {cnPageIndex + 1} / {cnTotalPages} 页
          </div>
          <div className="flex items-center gap-2">
            <Select
              value={String(cnPageSize)}
              onValueChange={(v) => {
                setCnPageSize(Number(v));
                setCnPageIndex(0);
              }}
            >
              <SelectTrigger className="h-8 w-[80px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {[10, 20, 50].map((s) => (
                  <SelectItem key={s} value={String(s)}>
                    {s} 条
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setCnPageIndex((i) => Math.max(0, i - 1))}
              disabled={cnPageIndex === 0}
            >
              上一页
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setCnPageIndex((i) => Math.min(cnTotalPages - 1, i + 1))}
              disabled={cnPageIndex >= cnTotalPages - 1}
            >
              下一页
            </Button>
          </div>
        </div>
      </div>
    );
  }

  if (market === "reits") {
    if (reitsLoading) {
      return (
        <div className="flex items-center justify-center py-16">
          <Icons.Spinner className="text-muted-foreground h-6 w-6 animate-spin" />
        </div>
      );
    }

    if (reitsIpos.length === 0) {
      return (
        <div className="flex items-center justify-center py-16">
          <EmptyPlaceholder
            icon={<Icons.TrendingUp className="text-muted-foreground h-10 w-10" />}
            title="REITs 数据暂不可用"
            description="集思录 A股 REITs 数据当前对未登录会员受限，暂时无法抓取公开列表。"
          />
        </div>
      );
    }

    const reitsTotalPages = Math.max(1, Math.ceil(reitsIpos.length / reitsPageSize));
    const reitsPagedIpos = reitsIpos.slice(
      reitsPageIndex * reitsPageSize,
      (reitsPageIndex + 1) * reitsPageSize,
    );
    return (
      <div className="space-y-2">
        <div className="overflow-x-auto rounded-md border">
          <Table className="min-w-[1200px]">
            <TableHeader>
              <TableRow>
                {REITS_COLUMNS.map((column) => (
                  <TableHead key={column}>{column}</TableHead>
                ))}
              </TableRow>
            </TableHeader>
            <TableBody>
              {reitsPagedIpos.map((record) => (
                <TableRow key={record.id}>
                  <TableCell className="font-medium">{record.code}</TableCell>
                  <TableCell>{record.name}</TableCell>
                  <TableCell>{record.assetType}</TableCell>
                  <TableCell>{record.subscriptionDate}</TableCell>
                  <TableCell>{record.issuePrice}</TableCell>
                  <TableCell>{record.subscriptionCode}</TableCell>
                  <TableCell>{record.maxSubscriptions}</TableCell>
                  <TableCell>{record.listingDate}</TableCell>
                  <TableCell>{record.underlyingAsset}</TableCell>
                  <TableCell>{record.sponsor}</TableCell>
                  <TableCell>
                    <Badge variant={STATUS_VARIANTS[record.status]}>
                      {STATUS_LABELS[record.status]}
                    </Badge>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
        <div className="flex items-center justify-between px-1">
          <div className="text-muted-foreground text-sm">
            共 {reitsIpos.length} 条，第 {reitsPageIndex + 1} / {reitsTotalPages} 页
          </div>
          <div className="flex items-center gap-2">
            <Select
              value={String(reitsPageSize)}
              onValueChange={(v) => {
                setReitsPageSize(Number(v));
                setReitsPageIndex(0);
              }}
            >
              <SelectTrigger className="h-8 w-[80px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {[10, 20, 50].map((s) => (
                  <SelectItem key={s} value={String(s)}>
                    {s} 条
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setReitsPageIndex((i) => Math.max(0, i - 1))}
              disabled={reitsPageIndex === 0}
            >
              上一页
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setReitsPageIndex((i) => Math.min(reitsTotalPages - 1, i + 1))}
              disabled={reitsPageIndex >= reitsTotalPages - 1}
            >
              下一页
            </Button>
          </div>
        </div>
      </div>
    );
  }

  if (market === "hk") {
    if (hkLoading) {
      return (
        <div className="flex items-center justify-center py-16">
          <Icons.Spinner className="text-muted-foreground h-6 w-6 animate-spin" />
        </div>
      );
    }
    const hkTotalPages = Math.max(1, Math.ceil(hkIpos.length / hkPageSize));
    const hkPagedIpos = hkIpos.slice(hkPageIndex * hkPageSize, (hkPageIndex + 1) * hkPageSize);
    return (
      <div className="space-y-2">
        <div className="overflow-x-auto rounded-md border">
          <Table className="min-w-[1400px]">
            <TableHeader>
              <TableRow>
                {HK_MARKET_COLUMNS.map((column) => (
                  <TableHead key={column}>{column}</TableHead>
                ))}
              </TableRow>
            </TableHeader>
            <TableBody>
              {hkPagedIpos.map((record) => (
                <TableRow key={record.id}>
                  <TableCell className="font-medium">{record.code}</TableCell>
                  <TableCell>{record.name}</TableCell>
                  <TableCell>{record.board}</TableCell>
                  <TableCell>{record.subscriptionStart}</TableCell>
                  <TableCell>{record.subscriptionEnd}</TableCell>
                  <TableCell>{getPreviousNthTradingDay(record.listingDate, 2)}</TableCell>
                  <TableCell>{getPreviousNthTradingDay(record.listingDate, 1)}</TableCell>
                  <TableCell>{record.listingDate}</TableCell>
                  <TableCell>{record.issuePrice}</TableCell>
                  <TableCell>{record.issueSize}</TableCell>
                  <TableCell>{record.lotSize}</TableCell>
                  <TableCell>{record.winRate}</TableCell>
                  <TableCell>{record.firstDayChange}</TableCell>
                  <TableCell>{record.underwriter}</TableCell>
                  <TableCell>
                    <Badge variant={STATUS_VARIANTS[record.status]}>
                      {STATUS_LABELS[record.status]}
                    </Badge>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
        <div className="flex items-center justify-between px-1 py-1">
          <p className="text-muted-foreground text-sm">
            共 {hkIpos.length} 条，第 {hkPageIndex + 1} / {hkTotalPages} 页
          </p>
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              <span className="text-sm">每页</span>
              <Select
                value={String(hkPageSize)}
                onValueChange={(v) => {
                  setHkPageSize(Number(v));
                  setHkPageIndex(0);
                }}
              >
                <SelectTrigger className="h-8 w-[70px]">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent side="top">
                  {[10, 20, 50].map((s) => (
                    <SelectItem key={s} value={String(s)}>
                      {s} 条
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="flex items-center gap-1">
              <Button
                variant="outline"
                size="icon"
                className="h-8 w-8"
                onClick={() => setHkPageIndex(0)}
                disabled={hkPageIndex === 0}
              >
                <Icons.ChevronsLeft className="h-4 w-4" />
              </Button>
              <Button
                variant="outline"
                size="icon"
                className="h-8 w-8"
                onClick={() => setHkPageIndex((p) => Math.max(0, p - 1))}
                disabled={hkPageIndex === 0}
              >
                <Icons.ChevronLeft className="h-4 w-4" />
              </Button>
              <Button
                variant="outline"
                size="icon"
                className="h-8 w-8"
                onClick={() => setHkPageIndex((p) => Math.min(hkTotalPages - 1, p + 1))}
                disabled={hkPageIndex >= hkTotalPages - 1}
              >
                <Icons.ChevronRight className="h-4 w-4" />
              </Button>
              <Button
                variant="outline"
                size="icon"
                className="h-8 w-8"
                onClick={() => setHkPageIndex(hkTotalPages - 1)}
                disabled={hkPageIndex >= hkTotalPages - 1}
              >
                <Icons.ChevronsRight className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </div>
      </div>
    );
  }

  const columns = MARKET_COLUMNS[market];
  const records = DEMO_DATA[market];

  if (records.length === 0) {
    return (
      <div className="flex items-center justify-center py-16">
        <EmptyPlaceholder
          icon={<Icons.TrendingUp className="text-muted-foreground h-10 w-10" />}
          title="暂无打新记录"
          description="当前市场暂无 IPO 认购记录，敬请期待。"
        />
      </div>
    );
  }

  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            {columns.map((col) => (
              <TableHead key={col}>{col}</TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {records.map((record) => (
            <TableRow key={record.id}>
              <TableCell className="font-medium">{record.name}</TableCell>
              <TableCell>{record.symbol}</TableCell>
              <TableCell>{record.subscriptionStart}</TableCell>
              <TableCell>{record.subscriptionEnd}</TableCell>
              <TableCell>{record.issuePrice}</TableCell>
              <TableCell>{record.ipoDate}</TableCell>
              <TableCell>
                <Badge variant={STATUS_VARIANTS[record.status]}>
                  {STATUS_LABELS[record.status]}
                </Badge>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
};

const IpoPage = () => {
  const views: SwipablePageView[] = useMemo(
    () => [
      {
        value: "hk",
        label: "港股打新",
        icon: Icons.TrendingUp,
        content: <IpoTable market="hk" />,
      },
      {
        value: "cn",
        label: "沪深京打新",
        icon: Icons.BarChart,
        content: <IpoTable market="cn" />,
      },
      {
        value: "cb",
        label: "可转债打新",
        icon: Icons.Briefcase,
        content: <IpoTable market="cb" />,
      },
      {
        value: "reits",
        label: "REITs打新",
        icon: Icons.Building,
        content: <IpoTable market="reits" />,
      },
      {
        value: "us",
        label: "美股打新",
        icon: Icons.PieChart,
        content: <IpoTable market="us" />,
      },
    ],
    [],
  );

  return <SwipablePage views={views} defaultView="hk" />;
};

export default IpoPage;
