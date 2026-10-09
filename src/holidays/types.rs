use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HolidayScope {
    National,
    State,
    City,
    Health,
}

impl HolidayScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::National => "national",
            Self::State => "state",
            Self::City => "city",
            Self::Health => "health",
        }
    }

    pub fn parse_list(raw: &str) -> Result<Vec<Self>, String> {
        let mut out = Vec::new();
        for part in raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            out.push(Self::parse_one(part)?);
        }
        if out.is_empty() {
            return Err("scopes vazio".into());
        }
        Ok(out)
    }

    fn parse_one(raw: &str) -> Result<Self, String> {
        match raw.to_ascii_lowercase().as_str() {
            "national" | "nacional" => Ok(Self::National),
            "state" | "estadual" => Ok(Self::State),
            "city" | "municipal" | "cidade" => Ok(Self::City),
            "health" | "saude" | "saúde" => Ok(Self::Health),
            other => Err(format!("scope inválido: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HolidayKind {
    /// Feriado legal (não laborável).
    Feriado,
    /// Ponto facultativo / tradição (Carnaval, etc.).
    Facultativo,
    /// Data comemorativa (calendário da saúde, etc.) — não bloqueia.
    Commemorative,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Holiday {
    pub date: NaiveDate,
    pub title: String,
    pub description: String,
    pub legislation: String,
    pub scope: HolidayScope,
    pub kind: HolidayKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_ibge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_color: Option<String>,
}

impl Holiday {
    pub fn national(
        date: NaiveDate,
        title: &str,
        description: &str,
        legislation: &str,
        kind: HolidayKind,
    ) -> Self {
        Self {
            date,
            title: title.into(),
            description: description.into(),
            legislation: legislation.into(),
            scope: HolidayScope::National,
            kind,
            uf: None,
            city_ibge: None,
            category: None,
            category_color: None,
        }
    }
}
