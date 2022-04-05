initSidebarItems({"fn":[["get_time_for_move","Calculates a time which should be allocated for the next move, based on `move_number`, `total_time`, `inc_time` (0 if not available) and `moves_to_go` (0 if not available). Formula and chart used when `moves_to_go` is zeroed can be found in the `/misc/time.xlsx` Excel sheet, but in general outline it tries to allocate more time during mid-game where usually there’s a lot of pieces on the board and it’s crucial to find some advantage at this phase. Formula used when `moves_to_go` is greater than zero is simpler and allocates time evenly."]]});