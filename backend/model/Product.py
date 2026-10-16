class Product():
    """Products and subproducts of a yield analysis"""
    def __init__(self,name:str, weight:float, purchase_price:float=0, ratio_percentage:float=100, yield_loss_percentage:float = 0) -> None:
        self.name:str = name
        self.purchase_price:float = purchase_price
        self.gross_weight:float = weight
        self.ratio_percentage:float= ratio_percentage
        self.yield_loss_percentage:float = yield_loss_percentage
        
        self.provisional_yield_loss_percentage:float = yield_loss_percentage # Provisional weight used to follow the ratio percentage distribution when dividing yield_loss_percentage weight between byproducts
        
        self.ratio_value:float = self.ratio_percentage/100 # ratio_percentage is the value provided by user defining the percentage value of a byproduct.

        self.parent:Product|None = None
        self.byproducts:list[Product] = []

        # A product can have yield loss event without byproducts, so if no parent, yield loss is calculated on own weight
        self.trim_loss:float = (self.parent.weight * self.yield_loss_percentage)/100 if self.parent is not None else (self.gross_weight * self.yield_loss_percentage)/100

        self.net_weight:float = self.gross_weight - self.trim_loss
        
        # If no parent, own weight is 100%
        self.weight_ratio: float = weight/self.parent.weight if self.parent is not None else 1
        
        # If no parent, own cost is 100%
        self.cost:float = self.parent.purchase_price * self.ratio_value if self.parent is not None else self.purchase_price

    # TODO: Check whether @classmethods are cleaner solution
    def add_byproduct(self, name:str, weight:float, percentage:float) -> int:
        total_percentages:float = sum(product.ratio_percentage for product in self.byproducts) + self.yield_loss_percentage
        if total_percentages > 100:
            return -1
        self.byproducts.append(Product(
                                            name=name,
                                            weight=weight,
                                            purchase_price=(weight*self.purchase_price)/self.gross_weight,
                                            ratio_percentage=percentage 
                                        ))
        return 0
    
    def set_yield_loss_percentage(self,percentage:int) -> None:
        self.yield_loss_percentage = percentage
        self.provisional_yield_loss_percentage = percentage

    # TODO: Check whether this is a good solution and how problem needs to be solved.
    def add_from_yield(self) -> None:
        if self.provisional_yield_loss_percentage <=0:
            return
        
        #substract percentage from yield
        # add percentage to byproduct from list

    



