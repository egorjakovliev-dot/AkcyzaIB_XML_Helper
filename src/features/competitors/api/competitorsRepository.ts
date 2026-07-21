import { api, type CompetitorInput } from '../../../shared/api/client';
export const competitorsRepository = { list: api.listCompetitors, create: (input: CompetitorInput) => api.createCompetitor(input) };
