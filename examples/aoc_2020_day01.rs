use itertools::Itertools;

use aoc::solution::Result;
use aoc::Solution;

struct Day01;

const INPUT: &'static str  ="1535\n1908\n1783\n1163\n1472\n1809\n1566\n1919\n1562\n1532\n1728\n1999\n1942\n337\n1135\n2006\n1083\n1483\n1688\n1511\n1134\n1558\n1139\n1790\n1406\n1255\n1627\n1941\n1619\n2009\n1453\n1806\n1756\n1634\n1026\n1847\n1520\n1914\n1836\n1440\n1839\n1527\n1638\n1642\n1776\n1148\n1958\n1616\n1952\n1092\n1081\n1898\n1487\n2000\n1921\n1579\n54\n1031\n1842\n1006\n1781\n1964\n168\n1339\n1094\n1997\n1522\n1962\n1837\n1730\n1244\n1593\n1752\n1400\n1330\n1649\n1639\n1493\n1696\n2003\n1612\n1717\n1835\n861\n1950\n1896\n557\n1926\n571\n1725\n1229\n1213\n1625\n1553\n1204\n1459\n1666\n1723\n1118\n1845\n1663\n1829\n1929\n1880\n1738\n1887\n1605\n1273\n1759\n1932\n1156\n1712\n1767\n1241\n1159\n1476\n1705\n1768\n1680\n1543\n2010\n1849\n1289\n1636\n1894\n1823\n1706\n1239\n1802\n1744\n1584\n1690\n1758\n1618\n1749\n1521\n1594\n1960\n1479\n1022\n1559\n1106\n1755\n1254\n1878\n1243\n1418\n1671\n1895\n1120\n1673\n1719\n1904\n724\n1945\n1940\n1819\n1939\n1103\n2008\n1791\n1874\n1544\n1892\n1557\n1617\n1998\n1641\n1907\n1563\n1089\n1086\n1276\n1591\n1614\n1216\n1658\n1514\n1899\n1760\n1797\n1831\n277\n1622\n1795\n1468\n1537\n1742\n1709\n1886\n1846\n1567\n1492\n1549\n1587\n1818\n1687\n1404\n1778\n1096\n";

impl Solution for Day01 {
    const TITLE: &'static str = "Report Repair";
    const DAY: u8 = 1;
    type Input = Vec<usize>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> Result<Self::Input> {
        Ok(input.lines().filter_map(|line| line.parse().ok()).collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input
            .iter()
            .cartesian_product(input.iter())
            .find(|(a, b)| *a + *b == 2020)
            .map(|(a, b)| a * b)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        input
            .iter()
            .cartesian_product(input.iter())
            .cartesian_product(input.iter())
            .find(|((a, b), c)| *a + *b + *c == 2020)
            .map(|((a, b), c)| a * b * c)
    }

    fn get_input() -> Result<String> {
        Ok(INPUT.to_owned())
    }
}

fn main() {
    aoc::solution!(Day01)
}

#[cfg(test)]
mod tests {
    use crate::Day01 as day_01;
    use crate::*;

    aoc::test_common!(day_01);

    aoc::test! {
        day_01:
        - "1721\n979\n366\n299\n675\n1456"
            => Some(514579)
            => Some(241861950)
    }
}
