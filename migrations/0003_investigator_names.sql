-- The investigators now bear names from Lovecraft's stories and the Cthulhu games.
UPDATE investigators SET code = CASE code
    WHEN 'poirot' THEN 'armitage'
    WHEN 'hercule' THEN 'carter'
    WHEN 'hastings' THEN 'malone'
    WHEN 'captain' THEN 'johansen'
    WHEN 'sandy' THEN 'barnes'
    WHEN 'lemon' THEN 'walker'
    ELSE code
END;
