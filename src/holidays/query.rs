use chrono::{Datelike, NaiveDate};

use super::city::{city_for_year, normalize_city_ibge};
use super::health::health_for_year;
use super::national::national_for_year;
use super::state::{normalize_uf, state_for_year};
use super::types::{Holiday, HolidayScope};

#[derive(Debug, Clone)]
pub struct HolidayQuery {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub scopes: Vec<HolidayScope>,
    pub uf: Option<String>,
    pub city_ibge: Option<String>,
}

pub fn list_holidays(query: &HolidayQuery) -> Result<Vec<Holiday>, String> {
    if query.to < query.from {
        return Err("to deve ser >= from".into());
    }
    if query.to > query.from + chrono::Duration::days(366 * 5) {
        return Err("intervalo máximo de 5 anos".into());
    }

    let years: Vec<i32> = (query.from.year()..=query.to.year()).collect();
    let mut out = Vec::new();

    for year in years {
        for scope in &query.scopes {
            match scope {
                HolidayScope::National => out.extend(national_for_year(year)),
                HolidayScope::State => {
                    let Some(uf) = query.uf.as_deref() else {
                        return Err("uf é obrigatório quando scopes inclui state".into());
                    };
                    let Some(uf) = normalize_uf(uf) else {
                        return Err(format!("uf inválida: {uf}"));
                    };
                    out.extend(state_for_year(&uf, year));
                }
                HolidayScope::City => {
                    let Some(city) = query.city_ibge.as_deref() else {
                        return Err("city_ibge é obrigatório quando scopes inclui city".into());
                    };
                    let Some(city) = normalize_city_ibge(city) else {
                        return Err(format!("city_ibge inválido: {city}"));
                    };
                    out.extend(city_for_year(&city, year));
                }
                HolidayScope::Health => out.extend(health_for_year(year)),
            }
        }
    }

    out.retain(|h| h.date >= query.from && h.date <= query.to);
    out.sort_by(|a, b| a.date.cmp(&b.date).then_with(|| a.title.cmp(&b.title)));
    out.dedup_by(|a, b| a.date == b.date && a.title == b.title && a.scope == b.scope);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_filters_and_computes_year() {
        let items = list_holidays(&HolidayQuery {
            from: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 4, 30).unwrap(),
            scopes: vec![HolidayScope::National],
            uf: None,
            city_ibge: None,
        })
        .unwrap();
        assert!(items.iter().any(|h| h.title == "Carnaval"));
        assert!(items.iter().any(|h| h.title == "Sexta-Feira Santa"));
        assert!(items.iter().all(|h| h.date.year() == 2026));
    }

    #[test]
    fn state_requires_uf() {
        let err = list_holidays(&HolidayQuery {
            from: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(),
            scopes: vec![HolidayScope::State],
            uf: None,
            city_ibge: None,
        })
        .unwrap_err();
        assert!(err.contains("uf"));
    }
}
